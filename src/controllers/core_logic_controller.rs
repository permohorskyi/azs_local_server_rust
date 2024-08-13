use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, web};
use crate::models::{get_nowtime_str, MyError};
use crate::StateDb;

type AsyncHandler = Arc<dyn Fn(web::Data<StateDb>) -> Pin<Box<dyn Future<Output = Result<HttpResponse, MyError>>>> + Send + Sync>;
#[get("")]
pub async fn m_global_main()->Result<HttpResponse, MyError>{
    let response = HttpResponse::Found()
        .insert_header((http::header::LOCATION, "/view/userspace/old/main"))
        .finish();
    Ok(response)
}
pub fn wrap_handler<F, Fut>(f: F) -> AsyncHandler
    where
        F: Fn(web::Data<StateDb>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<HttpResponse, MyError>> + 'static,
{
    Arc::new(move |state| Box::pin(f(state)))
}
pub async fn start_controller(admin_fn:AsyncHandler,user_fn:AsyncHandler,req:&HttpRequest,web_state:web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(admin) = req.extensions().get::<bool>(){
        if *admin==true {
            (admin_fn)(web_state.clone()).await
        }else{
            (user_fn)(web_state.clone()).await
        }
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        Err(MyError::DatabaseError(str_error))
    }
}