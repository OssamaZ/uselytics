mod config;
mod error;
mod ip;
mod response;

use axum::{
  extract::{Query, State},
  routing::get,
  Json, Router,
};
use axum_client_ip::{InsecureClientIp, SecureClientIpSource};
use axum_extra::{headers::UserAgent, TypedHeader};
use clap::Parser;
use config::Config;
use error::{AppError, AppResult};
use ip::{GeoResponseClient, IPParser};
use response::AppResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use uaparser_rs::{Client, UAParser};

struct AppState {
  db_pool: sqlx::PgPool,
  uaparser: UAParser,
  ipparser: IPParser,
}

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = Config::parse();
  println!("{config:?}");

  // database
  let db_pool = PgPoolOptions::new()
    .max_connections(25)
    .connect(&config.database_url)
    .await
    .expect("Unable to connect to the database.");
  // uaparser
  let uaparser = UAParser::from_yaml("./regexes.yaml").expect("Unable to read ua parser regexes.");
  // iplookup
  let ipparser = IPParser::new();

  // state
  let shared_state = Arc::new(AppState {
    db_pool,
    uaparser,
    ipparser,
  });

  let router = Router::new()
    .route("/status", get(status_handler))
    .route("/t", get(test_h))
    .route("/", get(handler))
    .with_state(shared_state)
    .layer(SecureClientIpSource::ConnectInfo.into_extension());

  let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  println!("listening on {}", listener.local_addr().unwrap());
  axum::serve(
    listener,
    router.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await
  .unwrap();
}

async fn status_handler(State(s): State<Arc<AppState>>) -> String {
  let result = sqlx::query!("select 'UP' as result")
    .fetch_one(&s.db_pool)
    .await;

  let result = result.map_or("Down".to_owned(), |r| r.result.unwrap_or("Down".into()));

  result
}

#[derive(Serialize)]
struct UAResponse {
  user_agent: String,
  client: UAResponseClient,
  ip: String,
  geo: Option<GeoResponseClient>,
  result: String,
}

#[derive(Serialize)]
struct UAResponseClient {
  browser: String,
  os: String,
  device: String,
}

async fn handler(
  InsecureClientIp(ip): InsecureClientIp,
  TypedHeader(ua): TypedHeader<UserAgent>,
  State(s): State<Arc<AppState>>,
) -> Json<UAResponse> {
  let result = sqlx::query!("select 1 + 1 as result")
    .fetch_one(&s.db_pool)
    .await;

  let result = result.map_or("Down".to_owned(), |r| {
    r.result.map_or("Hmm".into(), |v| format!("{v}"))
  });

  let Client {
    os,
    user_agent,
    device,
  } = s.uaparser.parse(ua.as_str());

  let geo = s.ipparser.parse(ip);

  Json(UAResponse {
    user_agent: ua.as_str().into(),
    client: UAResponseClient {
      browser: user_agent.family,
      os: os.family,
      device: device.family,
    },
    ip: ip.to_string(),
    geo: geo,
    result,
  })
}

#[derive(Deserialize)]
struct TestHQ {
  e: Option<usize>,
}

async fn test_h(Query(q): Query<TestHQ>) -> AppResult {
  let n = match q.e {
    Some(n) if n >= 1 && n <= 3 => n,
    _ => {
      return Err(AppError::BadRequest(
        "E must be a number between 1 and 3".to_string(),
      ))
    }
  };

  Ok(AppResponse(json!({ "val": n })))
}
