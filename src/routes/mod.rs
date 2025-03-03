use actix_web::web;
pub mod task_routes;
use task_routes::configure_task_routes;
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/api").configure(configure_task_routes));
}
