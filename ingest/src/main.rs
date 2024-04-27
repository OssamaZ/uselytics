mod config;

use axum::{extract::State, routing::get, Json, Router};
use axum_client_ip::{InsecureClientIp, SecureClientIpSource};
use axum_extra::{headers::UserAgent, TypedHeader};
use clap::Parser;
use config::Config;
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use uaparser_rs::{Client, UAParser};

struct AppState {
  uaparser: UAParser,
  iplookup: maxminddb::Reader<Vec<u8>>,
}

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = Config::parse();
  println!("{config:?}");

  // uaparser
  let uap = UAParser::from_yaml("./regexes.yaml").expect("Unable to read ua parser regexes.");
  // geolookup
  let mm = maxminddb::Reader::open_readfile("./geo.mmdb").expect("Unable to read mm db file.");

  // state
  let shared_state = Arc::new(AppState {
    uaparser: uap,
    iplookup: mm,
  });

  let router = Router::new()
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

#[derive(Serialize)]
struct UAResponse {
  user_agent: String,
  client: UAResponseClient,
  ip: String,
  geo: Option<GeoResponseClient>,
}

#[derive(Serialize)]
struct UAResponseClient {
  browser: String,
  os: String,
  device: String,
}

#[derive(Serialize)]
struct GeoResponseClient {
  pub latitude: f64,
  pub longitude: f64,
  pub postal_code: String,
  pub continent_code: String,
  pub continent_name: String,
  pub country_code: String,
  pub country_name: String,
  pub region_code: String,
  pub region_name: String,
  pub province_code: String,
  pub province_name: String,
  pub city_name: String,
  pub timezone: String,
}

async fn handler(
  InsecureClientIp(ip): InsecureClientIp,
  TypedHeader(ua): TypedHeader<UserAgent>,
  State(s): State<Arc<AppState>>,
) -> Json<UAResponse> {
  let Client {
    os,
    user_agent,
    device,
  } = s.uaparser.parse(ua.as_str());

  let lookup: Result<maxminddb::geoip2::City, maxminddb::MaxMindDBError> = s.iplookup.lookup(ip);
  let geo = match lookup {
    Ok(geoip) => {
      let region = geoip
        .subdivisions
        .as_ref()
        .filter(|subdivs| !subdivs.is_empty())
        .and_then(|subdivs| subdivs.get(0));

      let province = geoip
        .subdivisions
        .as_ref()
        .filter(|subdivs| subdivs.len() > 1)
        .and_then(|subdivs| subdivs.get(1));

      let (latitude, longitude) = match geoip.location {
        None => (0.0, 0.0),
        Some(ref v) => (v.latitude.unwrap_or(0.0), v.longitude.unwrap_or(0.0)),
      };

      let res = GeoResponseClient {
        latitude,
        longitude,
        postal_code: geoip
          .postal
          .and_then(|postal| postal.code)
          .unwrap_or("")
          .to_owned(),
        continent_code: geoip
          .continent
          .as_ref()
          .and_then(|cont| cont.code)
          .unwrap_or("")
          .to_owned(),
        continent_name: geoip
          .continent
          .as_ref()
          .and_then(|cont| cont.names.as_ref())
          .and_then(|names| names.get("en"))
          .map(|&val| val.to_owned())
          .unwrap_or("".to_owned()),
        country_code: geoip
          .country
          .as_ref()
          .and_then(|country| country.iso_code)
          .unwrap_or("")
          .to_owned(),
        country_name: geoip
          .country
          .as_ref()
          .and_then(|country| country.names.as_ref())
          .and_then(|names| names.get("en"))
          .map(|&val| val.to_owned())
          .unwrap_or("".to_owned()),
        region_code: region
          .and_then(|subdiv| subdiv.iso_code)
          .unwrap_or("")
          .to_owned(),
        region_name: region
          .as_ref()
          .and_then(|subdiv| subdiv.names.as_ref())
          .and_then(|names| names.get("en"))
          .map(|&val| val.to_owned())
          .unwrap_or("".to_owned()),
        province_code: province
          .as_ref()
          .and_then(|subdiv| subdiv.iso_code)
          .unwrap_or("")
          .to_owned(),
        province_name: province
          .as_ref()
          .and_then(|subdiv| subdiv.names.as_ref())
          .and_then(|names| names.get("en"))
          .map(|&val| val.to_owned())
          .unwrap_or("".to_owned()),
        city_name: geoip
          .city
          .as_ref()
          .and_then(|city| city.names.as_ref())
          .and_then(|names| names.get("en"))
          .map(|&val| val.to_owned())
          .unwrap_or("".to_owned()),
        timezone: geoip
          .location
          .and_then(|loc| loc.time_zone)
          .unwrap_or("")
          .to_owned(),
      };
      Some(res)
    }
    Err(_) => None,
  };

  Json(UAResponse {
    user_agent: ua.as_str().into(),
    client: UAResponseClient {
      browser: user_agent.family,
      os: os.family,
      device: device.family,
    },
    ip: ip.to_string(),
    geo: geo,
  })
}
