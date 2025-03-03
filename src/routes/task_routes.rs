use crate::handlers::task_handlers;
use actix_web::web;
pub fn configure_task_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/tasks").route("", web::get().to(task_handlers::hello)));
}
