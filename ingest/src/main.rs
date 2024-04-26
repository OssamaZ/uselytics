mod config;

use axum::{extract::State, routing::get, Json, Router};
use axum_extra::{headers::UserAgent, TypedHeader};
use clap::Parser;
use config::Config;
use serde::Serialize;
use std::sync::Arc;
use uaparser_rs::{Client, UAParser};

struct AppState {
  uaparser: UAParser,
}

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = Config::parse();
  println!("{config:?}");

  // uaparser
  let uap = UAParser::from_yaml("./regexes.yaml").expect("Unable to read ua parser regexes.");

  // state
  let shared_state = Arc::new(AppState { uaparser: uap });

  let router = Router::new()
    .route("/", get(handler))
    .with_state(shared_state);
  let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port))
    .await
    .unwrap();
  println!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, router).await.unwrap();
}

#[derive(Serialize)]
struct UAResponse {
  user_agent: String,
  client: UAResponseClient,
}

#[derive(Serialize)]
struct UAResponseClient {
  browser: String,
  os: String,
  device: String,
}

async fn handler(
  TypedHeader(ua): TypedHeader<UserAgent>,
  State(s): State<Arc<AppState>>,
) -> Json<UAResponse> {
  let Client {
    os,
    user_agent,
    device,
  } = s.uaparser.parse(ua.as_str());
  Json(UAResponse {
    user_agent: ua.as_str().into(),
    client: UAResponseClient {
      browser: user_agent.family,
      os: os.family,
      device: device.family,
    },
  })
}
