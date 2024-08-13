
mod no_cache_middleware;
mod models;
mod render_temps;
mod check_db_view_middleware;
mod swagger_docs;
mod base;
mod controllers;
mod globals;
mod check_db_api_middleware;
mod jwt;
mod check_auth_middleware;
mod check_auth_only_admin_middleware;


use std::sync::Arc;
use sqlx::{Error as SqlxError, Error, MySql, MySqlPool, Pool, SqlitePool};
use actix_files as fs;
use no_cache_middleware::NoCache;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::migrate::MigrateDatabase;
use tokio::sync::Mutex;
use std::sync;
use crate::controllers::*;
use crate::check_db_view_middleware::CheckDbView;
use crate::models::{AzsDb, get_nowtime_str, local_getMysqlInfo, local_io_getMysqlInfo, local_io_initDb, MyError, MysqlInfo};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::check_auth_middleware::CheckAuth;
use crate::check_auth_only_admin_middleware::CheckAuthOnlyAdmin;
use crate::check_db_api_middleware::CheckDbApi;
//use crate::logger::LogManager;
use crate::swagger_docs::ApiDoc;
use actix_cors::Cors;


struct StateDb{
    azs_db:Arc<Mutex<AzsDb>>,
    sqlite:SqlitePool,
}
async fn connect_db(db_url:&str) -> Result<SqlitePool,Error> {

    if !sqlx::Sqlite::database_exists(&db_url).await? {
        sqlx::Sqlite::create_database(&db_url).await?;
    }

    // Connect to the database
    let db = SqlitePool::connect(&db_url).await?;
    Ok(db)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sqlite= connect_db("azs_db.db").await.map_err(|e|

        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    )?;
    println!("Open local database");
    local_io_initDb(&sqlite).await?;
    let mysql_info=local_io_getMysqlInfo(&sqlite).await?;
    let mut azs_db=AzsDb::new();
    let res_conn=azs_db.connect(mysql_info,&sqlite).await;
    match res_conn {
        Ok(_) => {}
        Err(e) => {e.pushlog().await;}
    }
    let state=web::Data::new(StateDb{
        azs_db:Arc::new(Mutex::new(azs_db)),
        sqlite:sqlite,
    });
    println!("START WEB SERVER");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::clone(&state))
            .wrap(NoCache)
            .service(
                web::scope("/").wrap(CheckDbView).wrap(CheckAuth)
                .service(core_logic_controller::m_global_main)
            )
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(
                web::scope("/view")
                    .wrap(CheckDbView)
                        .service(view_old_controller::m_login)
                    .service(
                        web::scope("/userspace")
                            .wrap(CheckAuth)
                            .service(
                                web::scope("/old")
                                    .service(view_old_controller::m_main)
                                    .service(view_old_controller::m_main_settings)

                            )
                            .service(view_old_controller::m_main_settings_configuration)
                    )


            )
            .service(fs::Files::new("/public", "./azs_site/public").show_files_listing())
            .service(
                web::scope("/settings")
                    .service(settings_controller::m_show_error)
                    .service(settings_controller::m_show_properties)
            )
            .service(
                web::scope("/api/service")
                    .service(api_service_controller::m_check_db_connect)
                    .service(api_service_controller::m_set_db_properties)
                    .service(api_service_controller::m_out_auth)

            )
            .service(
                web::scope("/api/db")
                    .wrap(CheckDbApi)
                        .service(api_db_controller::m_test_request)
                        .service(api_db_controller::m_auth)
                    .service(
                        web::scope("/userspace")
                            .service(
                                web::scope("/admin")
                                    .wrap(CheckAuthOnlyAdmin)
                                    .service(api_db_controller::m_save_trks_position)
                                    .service(api_db_controller::m_settings_eachobjects_set)
                                    .service(api_db_controller::m_settings_eachobjects_get)
                                    .service(api_db_controller::m_settings_eachobjects_delete)
                                    .service(api_db_controller::m_settings_set)
                                    .service(api_db_controller::m_settings_get)
                                    .service(api_db_controller::m_settings_delete)
                                    .service(api_db_controller::m_settings_tovar_set)
                                    .service(api_db_controller::m_settings_tank_set)
                                    .service(api_db_controller::m_settings_trk_set)
                                    .service(api_db_controller::m_settings_tovar_get)
                                    .service(api_db_controller::m_settings_tank_get)
                                    .service(api_db_controller::m_settings_trk_get)
                                    .service(api_db_controller::m_settings_tovar_id_get)
                                    .service(api_db_controller::m_settings_tank_id_get)
                                    .service(api_db_controller::m_settings_trk_id_get)
                            )
                            .service(
                                web::scope("/user")
                            )
                            .service(
                                web::scope("/all")
                                    .wrap(CheckAuth)
                                    .service(api_db_controller::m_out_shift)
                            )
                    )
            )
            .service(
                web::scope("/api/localdb")

            )
    })
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}