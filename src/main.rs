use actix_web::{App, HttpServer};
mod handlers;
mod routes;
mod models;
use crate::routes::configure_routes;
use dotenv::dotenv;
// use crate::db::postgres::Database;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Hello, world!");
  // let db = Database::new(&config).await.expect("Failed to connect to database");
  dotenv().expect("Failed to load .env file");
  let host = dotenv::var("host");
  let port = dotenv::var("port");
  HttpServer::new(move || {
    App::new()
      // .app_data(web::Data::new(db.pool.clone()))
      .configure(configure_routes)
  })
  .bind((host.unwrap(), port.unwrap().parse().unwrap()))?
  .run()
  .await
}
