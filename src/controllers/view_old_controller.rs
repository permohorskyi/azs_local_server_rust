use std::collections::HashMap;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use ramhorns::Template;
use crate::base::file_openString;
use crate::controllers::core_logic_controller::{start_controller, wrap_handler};
use crate::models::{AzsDb, get_nowtime_str, MyError, Pist, ScreenSize, Tank, Tovar, Trk};
use crate::render_temps::{AdminTemplate, AuthTemplate, ErrorDb, MainTemplate, MysqlInfowithErrorDb, PistForTemplate, TrkForTemplate};
use crate::{main, StateDb};
//BASE URL /view/old
#[get("/login")]
pub async fn m_login(state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
    let mut id_user_smena=0;
    let smena=AzsDb::getSmena(state.azs_db.clone()).await?;
    let mut users = AzsDb::getUsers(state.azs_db.clone()).await?;
    for i in 0..users.len(){
        if users[i].id_user==smena.id_user{
            let temp=users[0].clone();
            users[0]=users[i].clone();
            users[i]=temp;
            break;
        }
    }
    let auth = AuthTemplate {
        smena: smena.status,
        users: users
    };
    let contents = file_openString("./azs_site/public/old/login.html").await?;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&auth)))
}
//BASE URL /view/userspace/old
async fn get_trks_for_template(tovars: &Vec<Tovar>, tanks: &Vec<Tank>, trks: &Vec<Trk>,screen_size: ScreenSize) -> Result<Vec<TrkForTemplate>, MyError> {
    // Створення HashMap для швидкого доступу до Tovar по id_tovar
    let tovar_map: HashMap<i32, &Tovar> = tovars.iter().map(|t| (t.id_tovar, t)).collect();

    // Створення HashMap для швидкого доступу до Tank по id_tank
    let tank_map: HashMap<i32, &Tank> = tanks.iter().map(|t| (t.id_tank, t)).collect();

    let mut trks_template: Vec<TrkForTemplate> = Vec::with_capacity(trks.len());

    for trk in trks {
        let mut pists_: Vec<PistForTemplate> = Vec::with_capacity(trk.pists.len());

        for pist in &trk.pists {
            if let Some(tank) = tank_map.get(&pist.id_tank) {
                if let Some(tovar) = tovar_map.get(&tank.id_tovar) {
                    let pist_template = PistForTemplate {
                        id_pist: pist.id_pist,
                        id_tank: pist.id_tank,
                        price: tovar.price,
                        r: tovar.color.r,
                        g: tovar.color.g,
                        b: tovar.color.b,
                        name: tovar.name.clone()
                    };
                    pists_.push(pist_template);
                } else {
                    let str_error = format!("MYSQL|| {} error: PARSE TOVAR\n", get_nowtime_str());
                    return Err(MyError::DatabaseError(str_error));

                }
            } else {
                let str_error = format!("MYSQL|| {} error: PARSE TANKS\n", get_nowtime_str());
                return Err(MyError::DatabaseError(str_error));
            }
        }
        let x_pos_f=trk.x_pos as f32;
        let y_pos_f=trk.y_pos as f32;
        let screen_width_f=screen_size.width as f32;
        let trk_template = TrkForTemplate {
            nn: trk.nn,
            id_trk: trk.id_trk,
            x_pos: x_pos_f/screen_width_f*100.0,
            y_pos: y_pos_f/screen_width_f*100.0,
            scale: trk.scale,
            pists: pists_
        };
        trks_template.push(trk_template);
    }
    Ok(trks_template)
}
pub async fn a_main(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{
    let (screen_result, tovars_result, tanks_result, trks_result) = tokio::join!(AzsDb::getScreenSize(state.azs_db.clone()),
        AzsDb::getTovars(state.azs_db.clone()), AzsDb::getTanks(state.azs_db.clone()), AzsDb::getTrks(state.azs_db.clone()));
    let screen = screen_result?;
    let tovars = tovars_result?;
    let tanks = tanks_result?;
    let trks = trks_result?;
    let trks_for_template=get_trks_for_template(&tovars,&tanks,&trks,screen.clone()).await?;
    let main_template=MainTemplate{
        admin:true,
        screen_width:screen.width,
        trks:trks_for_template
    };
    let contents = file_openString("./azs_site/public/old/serv.html").await?;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&main_template)))
}
pub async fn u_main(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{

    let (screen_result, tovars_result, tanks_result, trks_result) = tokio::join!(AzsDb::getScreenSize(state.azs_db.clone()),
        AzsDb::getTovars(state.azs_db.clone()), AzsDb::getTanks(state.azs_db.clone()), AzsDb::getTrks(state.azs_db.clone()));
    let screen = screen_result?;
    let tovars = tovars_result?;
    let tanks = tanks_result?;
    let trks = trks_result?;
    let trks_for_template=get_trks_for_template(&tovars,&tanks,&trks,screen.clone()).await?;
    let main_template=MainTemplate{
        admin:false,
        screen_width:screen.width,
        trks:trks_for_template
    };
    let contents = file_openString("./azs_site/public/old/serv.html").await?;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&main_template)))
}

#[get("/main")]
pub async fn m_main(req:HttpRequest,state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
    start_controller(wrap_handler(a_main), wrap_handler(u_main),&req,state.clone()).await
}
pub async fn a_main_settings(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{
    let admin_template=AdminTemplate{admin:true};
    let contents = file_openString("./azs_site/public/old/settings_azs.html").await?;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&admin_template)))
}
pub async fn u_main_settings(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{
    let admin_template=AdminTemplate{admin:false};
    let contents = file_openString("./azs_site/public/old/settings_azs.html").await?;
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&admin_template)))
}

#[get("/main/settings")]
pub async fn m_main_settings(req:HttpRequest,state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
    start_controller(wrap_handler(a_main_settings), wrap_handler(u_main_settings),&req,state.clone()).await
}
pub fn get_http_redirect()->HttpResponse{
    let response = HttpResponse::Found()
        .insert_header((http::header::LOCATION, "/"))
        .finish();
    response
}
pub async fn a_main_settings_configuration(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{

    let contents = file_openString("./azs_site/index.html").await?;

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
pub async fn u_main_settings_configuration(state: web::Data<StateDb>)-> Result<HttpResponse, MyError>{
    Ok(get_http_redirect())
}
#[get("/main/settings/configuration")]
pub async fn m_main_settings_configuration(req:HttpRequest,state: web::Data<StateDb>) -> Result<HttpResponse, MyError> {
    start_controller(wrap_handler(a_main_settings_configuration), wrap_handler(u_main_settings_configuration),&req,state.clone()).await
}
