use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
  #[structopt(name = "Host name", long = "--host", default_value = "0.0.0.0")]
  pub host: String,

  #[structopt(name = "Port", long = "--port", default_value = "8088")]
  pub port: String,

  #[structopt(name = "Log Level", long = "--loglevel", default_value = "info")]
  pub log_level: String,
  #[structopt(name = "JWT Key", long = "--jwt-key", default_value = "")]
  pub jwt_key: String,

  #[structopt(
    name = "Database url",
    long = "--db-url",
    default_value = "postgres://quotes_api:password@localhost/quotes_api" // not actual URL used in prod
  )]
  pub database_url: String,
}
