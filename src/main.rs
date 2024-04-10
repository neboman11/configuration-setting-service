extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod configuration_settings;
mod database;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    database::init();

    let mut server = HttpServer::new(|| App::new().configure(configuration_settings::init_routes));

    let host = env::var("HOST").expect("Please set host in .env");
    let port = env::var("PORT").expect("Please set port in .env");
    server = server.bind(format!("{}:{}", host, port))?;

    server.run().await
}
