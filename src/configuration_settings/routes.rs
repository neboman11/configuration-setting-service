use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::configuration_settings::{ConfigurationSetting, ConfigurationSettings};
use crate::error_handler::CustomError;

#[utoipa::path(responses((status = OK, body = Vec<ConfigurationSetting>)))]
#[get("/configuration_settings")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let configuration_settings = web::block(|| ConfigurationSettings::find_all())
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(configuration_settings.unwrap()))
}

#[utoipa::path(responses((status = OK, body = Vec<ConfigurationSetting>)))]
#[get("/configuration_settings/{section}")]
async fn get_section(section: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let configuration_settings =
        web::block(|| ConfigurationSettings::get_section(section.into_inner()))
            .await
            .unwrap();
    Ok(HttpResponse::Ok().json(configuration_settings.unwrap()))
}

#[utoipa::path(responses((status = OK, body = ConfigurationSetting)))]
#[get("/configuration_setting/{id}")]
async fn find_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let configuration_setting = ConfigurationSettings::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(configuration_setting))
}

#[utoipa::path(responses((status = OK, body = ConfigurationSetting)))]
#[get("/configuration_setting/{section}/{name}")]
async fn find_by_section_and_name(
    params: web::Path<(String, String)>,
) -> Result<HttpResponse, CustomError> {
    let (section, name) = params.into_inner();
    let configuration_setting = ConfigurationSettings::find_by_section_and_name(section, name)?;
    Ok(HttpResponse::Ok().json(configuration_setting))
}

#[utoipa::path(responses((status = OK, body = ConfigurationSetting)))]
#[post("/configuration_setting")]
async fn create(
    configuration_setting: web::Json<ConfigurationSetting>,
) -> Result<HttpResponse, CustomError> {
    let configuration_setting = ConfigurationSettings::create(configuration_setting.into_inner())?;
    Ok(HttpResponse::Ok().json(configuration_setting))
}

#[utoipa::path(responses((status = OK, body = ConfigurationSetting)))]
#[put("/configuration_setting/{id}")]
async fn update(
    id: web::Path<i32>,
    configuration_setting: web::Json<ConfigurationSetting>,
) -> Result<HttpResponse, CustomError> {
    let configuration_setting =
        ConfigurationSettings::update(id.into_inner(), configuration_setting.into_inner())?;
    Ok(HttpResponse::Ok().json(configuration_setting))
}

#[utoipa::path(responses((status = OK, body = ConfigurationSetting)))]
#[put("/configuration_setting/{section}/{name}")]
async fn update_by_secion_and_name(
    params: web::Path<(String, String)>,
    configuration_setting: web::Json<ConfigurationSetting>,
) -> Result<HttpResponse, CustomError> {
    let (section, name) = params.into_inner();
    let configuration_setting = ConfigurationSettings::update_by_secion_and_name(
        section,
        name,
        configuration_setting.into_inner(),
    )?;
    Ok(HttpResponse::Ok().json(configuration_setting))
}

#[utoipa::path(responses((status = OK)))]
#[delete("/configuration_setting/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_configuration_setting = ConfigurationSettings::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_configuration_setting })))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(get_section);
    config.service(find_by_id);
    config.service(find_by_section_and_name);
    config.service(create);
    config.service(update);
    config.service(delete);
}
