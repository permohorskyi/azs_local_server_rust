use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};
use crate::models;
use crate::objects_of_controllers;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::api_db_controller::m_settings_eachobjects_get,
        super::api_db_controller::m_settings_eachobjects_set,
        super::api_db_controller::m_settings_eachobjects_delete,
        super::api_db_controller::m_settings_tovar_set,
        super::api_db_controller::m_settings_tank_set,
        super::api_db_controller::m_settings_trk_set,
        super::api_db_controller::m_settings_tovar_get,
        super::api_db_controller::m_settings_tank_get,
        super::api_db_controller::m_settings_trk_get,
        super::api_db_controller::m_settings_tovar_id_get,
        super::api_db_controller::m_settings_tank_id_get,
        super::api_db_controller::m_settings_trk_id_get
    ),
components(
    schemas(
        models::Tank,
        models::Pist,
        models::Trk,
        models::Tovar,
        models::Color,
        models::Tank_ID,
        models::Pist_ID,
        models::Trk_ID,
        models::Tovar_ID,
        objects_of_controllers::AllObject_ID,
        objects_of_controllers::RequestResult,
        objects_of_controllers::AllObject,
    )
),
tags((name = "BasicAPI", description = "A very Basic API")),

)]
pub struct ApiDoc;