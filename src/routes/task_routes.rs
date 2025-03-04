use crate::handlers::task_handlers;
use actix_web::web;
pub fn configure_task_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/tasks")
      .route("", web::get().to(task_handlers::hello))
      .route("", web::post().to(task_handlers::create_task))
      .service(
        web::scope("/{task_id}")
          .route("", web::get().to(task_handlers::hello))
          .route("", web::put().to(task_handlers::hello))
          .route("", web::delete().to(task_handlers::hello)),
      ),
  );
}
