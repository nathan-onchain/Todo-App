use actix_web::{get, web, HttpResponse, HttpRequest, HttpMessage};
use crate::{services::user_service, models::users::{RegisterUser, LoginUser}};
use crate::auth::jwt::Claims;
use serde_json::json;

pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    payload: web::Json<RegisterUser>,
) -> HttpResponse{
    match user_service::register_user(&pool, payload.into_inner()).await {
        Ok(token) => {
            println!("✅ Registration successful");
            HttpResponse::Ok().json(serde_json::json!({"token": token}))
        }
         Err(e) => {
            println!("❌ Registration failed: {:?}", e);
            HttpResponse::InternalServerError().json(json!({ "error": e.to_string() }))
        }
    }
}

pub async fn login(pool: web::Data<sqlx::PgPool>, payload: web::Json<LoginUser>) -> HttpResponse {
    match user_service::login_user(&pool, payload.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({"token": token})),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}


#[get("/get_me")]
pub async fn get_me(req: HttpRequest) -> HttpResponse {
    if let Some(user_id) = req.extensions().get::<String>() {
        HttpResponse::Ok().json(serde_json::json!({
            "message": "Access granted to protected route",
            "user_id": user_id
        }))
    } else {
        HttpResponse::Unauthorized().body("No claims found")
    }
}
