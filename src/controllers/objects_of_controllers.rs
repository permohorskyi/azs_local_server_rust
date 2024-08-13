use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::{ScreenSize, Tank, Tank_ID, Tovar, Tovar_ID, Trk, Trk_ID};

#[derive(Deserialize,Serialize,ToSchema)]
pub struct RequestResult{
    pub status:bool,
}
#[derive(Deserialize,Serialize)]
pub struct AuthInfo{
    pub id_user:i32,
    pub password:String
}
#[derive(Deserialize,Serialize)]
pub struct AuthResult{
    pub id_user:i32,
    pub password:String
}
#[derive(Deserialize,Serialize,ToSchema)]
pub struct AllObject{
    #[serde(default)]
    pub trks:Option<Vec<Trk>>,
    #[serde(default)]
    pub tovars:Option<Vec<Tovar>>,
    #[serde(default)]
    pub tanks:Option<Vec<Tank>>
}




#[derive(Deserialize,Serialize,ToSchema)]
pub struct AllObject_ID{
    #[serde(default)]
    pub trks:Option<Vec<Trk_ID>>,
    #[serde(default)]
    pub tovars:Option<Vec<Tovar_ID>>,
    #[serde(default)]
    pub tanks:Option<Vec<Tank_ID>>
}