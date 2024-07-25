use serde::Serialize;
use std::net::IpAddr;

pub struct IPParser {
  iplookup: maxminddb::Reader<Vec<u8>>,
}
#[derive(Serialize)]
pub struct GeoResponseClient {
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

impl IPParser {
  pub fn new() -> IPParser {
    // geolookup
    let iplookup =
      maxminddb::Reader::open_readfile("./geo.mmdb").expect("Unable to read mm db file.");
    IPParser { iplookup: iplookup }
  }

  pub fn parse(&self, ip: IpAddr) -> Option<GeoResponseClient> {
    let lookup: Result<maxminddb::geoip2::City, maxminddb::MaxMindDBError> =
      self.iplookup.lookup(ip);
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
    geo
  }
}
