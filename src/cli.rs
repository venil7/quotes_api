use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
  #[structopt(name = "hostname", long = "--host", default_value = "0.0.0.0")]
  pub host: String,

  #[structopt(name = "port", long = "--port", default_value = "8088")]
  pub port: String,

  #[structopt(name = "loglevel", long = "--loglevel", default_value = "info")]
  pub log_level: String,
}
