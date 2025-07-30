use actix_web::web;
use crate::controllers::user_controller::{register, login, get_me};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    )
    .service(get_me);
}