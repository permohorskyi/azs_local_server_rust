use std::future;
use std::future::{Future, ready, Ready};
use std::pin::Pin;
use std::rc::Rc;

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Either, Error, HttpMessage, HttpResponse, ResponseError, web};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use chrono::Utc;
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::error::DatabaseError;
use crate::jwt::{Claims, create_token, validate_token};
use crate::models::MyError;
use crate::StateDb;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct CheckAuthOnlyAdmin;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for CheckAuthOnlyAdmin
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>+ 'static,
        S::Future: 'static,
        B: 'static,
{

    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckAuthOnlyAdminMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckAuthOnlyAdminMiddleware { service: Rc::new(service) }))
    }
}

pub struct CheckAuthOnlyAdminMiddleware<S> {
    service: Rc<S>,
}
fn extract_cookie(req: &ServiceRequest, cookie_name: &str) -> Option<String> {
    req.headers()
        .get(header::COOKIE)
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find_map(|cookie| {
                    let mut parts = cookie.trim().splitn(2, '=');
                    if parts.next()? == cookie_name {
                        parts.next().map(|value| value.to_string())
                    } else {
                        None
                    }
                })
        })
}

impl<S, B> Service<ServiceRequest> for CheckAuthOnlyAdminMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>+ 'static,
        S::Future: 'static,
        B: 'static,
{

    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future =LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let state = req.app_data::<web::Data<StateDb>>().unwrap();
            println!("Hi from start. You requested: {}", req.path());
            let token=extract_cookie(&req,"refresh_token");
            let cookie = Cookie::build("refresh_token", "")
                .path("/")
                .http_only(true)
                .finish();
            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/view/login")).cookie(cookie)
                .finish().map_into_right_body();
            let azs_db=state.azs_db.lock().await;
            let mysql_info=azs_db.mysql_info_success.clone();
            drop(azs_db);
            match token {
                None => {

                    Ok(ServiceResponse::new(req.into_parts().0, response))
                }
                Some(some) => {
                    let mut is_admin=false;
                    if validate_token(some,&mut is_admin,mysql_info)==true {
                        if is_admin==true {
                            service.call(req).await.map(ServiceResponse::map_into_left_body)
                        }else{
                            Ok(ServiceResponse::new(req.into_parts().0, response))
                        }
                    }else{
                        Ok(ServiceResponse::new(req.into_parts().0, response))
                    }
                }
            }
            // if azs_db.mysql.is_none() {
            //     drop(azs_db);
            //     let response = HttpResponse::Found()
            //         .insert_header((http::header::LOCATION, "/settings/dbproperties"))
            //         .finish().map_into_right_body();
            //     Ok(ServiceResponse::new(req.into_parts().0, response))
            // } else {
            //     drop(azs_db);
            //     service.call(req).await.map(ServiceResponse::map_into_left_body)
            // }
        })

    }
}