use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_NAME"))]
pub struct Cli {
  #[structopt(name = "hostname", long = "--host", default_value = "127.0.0.1")]
  pub host: String,

  #[structopt(name = "port", long = "--port", default_value = "8088")]
  pub port: String,
}
