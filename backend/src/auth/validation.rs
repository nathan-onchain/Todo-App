use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;


#[derive(Deserialize)]
struct UsernameQuery {
    username: String,
}

#[get("/check-username")]
async fn check_username(
    query: web::Query<UsernameQuery>,
    db_pool: web::Data<PgPool>, // <-- pass in connection pool
) -> impl Responder {
    let username = query.username.to_lowercase();

    // Query Postgres to check if username exists
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM users 
            WHERE LOWER(username) = $1
        )
        "#,
        username
    )
    .fetch_one(db_pool.get_ref())
    .await
    .unwrap_or(Some(false));

    HttpResponse::Ok().json(serde_json::json!({
        "available": !exists.unwrap_or(false)
    }))
}
