use actix_web::web;
use crate::controllers::task_controller::create_task;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tasks")
            .route(web::post().to(create_task))
    );
}
use crate::routes::task_routes::config as task_routes;