#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate log;

use std::env;
use dotenv;
use actix_web::{App, HttpServer};
use env_logger;

mod schemas;
mod api_error;
mod database;
mod controllers;
mod models;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  if cfg!(debug_assertions) {
    env::set_var("RUST_LOG", "warn,info,error,debug");
    env_logger::init();
    dotenv::dotenv().ok().expect("Unable to read .env file");
    warn!("⚠️   Running in development mode");
  } else {
    env::set_var("RUST_LOG", "warn,error");
  }

  database::init();

  let host: String = env::var("HOST").expect("Missing HOST env var");
  let port: String = env::var("PORT").expect("Missing PORT env var");
  let target = format!("{}:{}", host, port);

  info!("🚀  Server ready at {}", format!("http://{}",&target));

  HttpServer::new(move ||
      App::new()
        .configure(routes::init_routes)
    )
    .bind(target)?
    .run()
    .await
}