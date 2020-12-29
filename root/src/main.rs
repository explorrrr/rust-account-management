use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod controllers;
mod config;
mod error;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let config = crate::config::Config::from_env().unwrap();
  let pool = config.pg.create_pool(NoTls).unwrap();

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .service(controllers::health_check::health_check)
  })
  .bind(config.server_addr.clone())?
  .run()
  .await
}
