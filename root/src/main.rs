use actix_web::{App, HttpServer};
use envy;
use serde::Deserialize;
use std::process;

// use tokio_postgres::NoTls;

mod controllers;
mod config;
mod error;
#[derive(Deserialize, Debug)]
struct Config {
  server_addr: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

  // 設定項目の取得
  let config: Config = match envy::from_env::<Config>() {
    Ok(val) => val,
    Err(err) => {
      println!("{}", err);
      process::exit(1);
    }
  };
  // let pool = config.pg.create_pool(NoTls).unwrap();

  HttpServer::new(move || {
    App::new()
      // .data(pool.clone())
      .service(controllers::health_check::health_check)
  })
  .bind(config.server_addr.clone())?
  .run()
  .await
}
