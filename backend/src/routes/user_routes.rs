use crate::auth::jwt_middleware::JwtMiddleware;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    )
    .service(
        web::scope("")
            .wrap(JwtMiddleware) // Protects this scope
            .route("/get_me", web::get().to(get_me))
    );
}
