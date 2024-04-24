use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
  #[arg(short, long = "port", env = "PORT", default_value = "3030")]
  /// The port for the server API
  pub port: u16,

  /// The postgres database url
  #[arg(
    short,
    long = "database_url",
    env = "DATABASE_URL",
    default_value = "postgres://user:password@localhost/uselytics"
  )]
  pub database_url: String,
}
