extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

mod configuration_settings;
mod database;
mod error_handler;
mod schema;

#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description"),
    paths(
        configuration_settings::routes::find_all,
        configuration_settings::routes::get_section,
        configuration_settings::routes::find_by_id,
        configuration_settings::routes::find_by_section_and_name,
        configuration_settings::routes::create,
        configuration_settings::routes::update,
        configuration_settings::routes::delete
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    database::init();

    let mut server = HttpServer::new(|| {
        let (server, mut _api) = App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .into_utoipa_app()
            .configure(configuration_settings::init_routes)
            .split_for_parts();
        server
    });

    let host = env::var("HOST").expect("Please set host in .env");
    let port = env::var("PORT").expect("Please set port in .env");
    server = server.bind(format!("{}:{}", host, port))?;

    server.run().await
}
