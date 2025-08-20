use crate::auth::jwt_middleware::JwtMiddleware;
use crate::controllers::user_controller::{register, login, get_me};
use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    )
    .service(
        web::scope("")
            .wrap(JwtMiddleware) // Protects this scope
            .service(get_me)
    );
}
