use crate::{
    models::tasks::{CreateTask, Task}, UpdateTask,
    utils::auth::get_user_id_from_request, // 👈 JWT decoder
};
use actix_web::{web, HttpResponse, Responder};  

use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_task(pool: web::Data<PgPool>, req: actix_web::HttpRequest, payload: web::Json<CreateTask>,) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(Uuid) => uuid::Uuid::parse_str(&uuid::Uuid::new_v4().to_string()).unwrap_or(uuid::Uuid::nil()),
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or missing JWT token"),
    };

    let task_id = Uuid::new_v4();
    let now = chrono::Utc::now().naive_utc();

    let query_result = sqlx::query_as!(
       Task,
        r#"
        INSERT INTO tasks (id, user_id, title, description, completed, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, title, description, completed, created_at
        "#,
        task_id,
        user_id,
        payload.title,
        payload.description,
        false,
        now 
    )
    .fetch_one(pool.get_ref())
    .await;

    match query_result {
        Ok(task) => {
            println!("✅ Task created successfully");
            HttpResponse::Created().json(task)
        }
        Err(e) => {
            println!("❌ Failed to create task: {:?}", e);
            HttpResponse::InternalServerError().json(json!({ "error": e.to_string() }))
        }
    }
}