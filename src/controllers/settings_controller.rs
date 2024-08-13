use actix_web::{get, HttpResponse, Responder, web};
use ramhorns::Template;
use log::error;
use crate::{render_temps, StateDb};
use crate::base::file_openString;
use crate::globals::LOGS_DB_ERROR;
use crate::models::MyError;

#[get("/dberror")]
pub async fn m_show_error(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>
{
    let azs_db=state.azs_db.lock().await;
    let error=render_temps::ErrorDb{error:LOGS_DB_ERROR.lock().await.clone()};
    let contents= file_openString("./azs_site/public/old/error_db.html").await;
    match contents {
        Ok(res) => {
            let tpl = Template::new(res).unwrap();
            Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&error)))
        }
        Err(err) => {
            drop(error);
            err.pushlog().await;
            Ok(HttpResponse::Ok().content_type("text/html").body(LOGS_DB_ERROR.lock().await.clone()))
        }
    }

}
#[get("/dbproperties")]
pub async fn m_show_properties(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>
{
    let azs_db=state.azs_db.lock().await;
    let ctx=render_temps::MysqlInfowithErrorDb{mysql_info_last:azs_db.mysql_info_last.clone(),mysql_info_success:azs_db.mysql_info_success.clone(),error_db:LOGS_DB_ERROR.lock().await.clone()};
    let contents= file_openString("./azs_site/public/old/settings_db_error.html").await;
    match contents {
        Ok(res) => {
            let tpl = Template::new(res).unwrap();
            Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&ctx)))
        }
        Err(err) => {
            err.pushlog().await;
            Ok(HttpResponse::Ok().content_type("text/html").body(LOGS_DB_ERROR.lock().await.clone()))
        }
    }

}
