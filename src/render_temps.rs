
use crate::models::{Color, MysqlInfo, Pist, User};
use ramhorns::{Template, Content};
#[derive(Content)]
pub struct AuthTemplate {
    pub smena:bool,
    pub users: Vec<User>,
}
#[derive(Content)]
pub struct AdminTemplate {
    pub admin:bool
}
#[derive(Content)]
pub struct PistForTemplate {
    pub id_pist: i32,
    pub id_tank: i32,
    pub price:f32,
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub name:String
}
#[derive(Content)]
pub struct TrkForTemplate{
    pub nn:i32,
    pub id_trk: i32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub scale: f64,
    pub pists: Vec<PistForTemplate>,
}
#[derive(Content)]
pub struct MainTemplate {
    pub admin:bool,
    pub screen_width:i32,
    pub trks:Vec<TrkForTemplate>
}
#[derive(Content)]
pub struct ErrorDb {
    pub error:String
}
#[derive(Content)]
pub struct MysqlInfowithErrorDb {
    pub mysql_info_last:MysqlInfo,
    pub mysql_info_success:MysqlInfo,
    pub error_db:String
}