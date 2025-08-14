use actix_web::{App, HttpServer, web};
use crate::routes::user_routes::user_routes;
use crate::config::load_env;
use crate::database::db::establish_connection;

mod config;
mod routes;
mod database;
mod controllers;
mod models;
mod services;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    let pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allow_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}