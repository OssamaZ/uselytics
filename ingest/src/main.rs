mod config;

use axum::{routing::get, Router};
use clap::Parser;
use config::Config;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = Config::parse();
  println!("{config:?}");

  let router = Router::new().route("/", get(handler));
  let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port))
    .await
    .unwrap();
  println!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, router).await.unwrap();
}

async fn handler() -> &'static str {
  "Hello World"
}
