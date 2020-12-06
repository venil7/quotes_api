use actix_web::{middleware, web, App, HttpServer};
use log::LevelFilter;
use quote_api::api::Api;
use quote_api::auth::bearer_validator;
use quote_api::auth_api::AuthApi;
use quote_api::cli;
use quote_api::handlers;
use simple_logger::SimpleLogger;
use std::str::FromStr;
use structopt::StructOpt;

use actix_web_httpauth::middleware::HttpAuthentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let cli = cli::Cli::from_args();

  SimpleLogger::new()
    .with_level(LevelFilter::from_str(&cli.log_level).unwrap_or(LevelFilter::Info))
    .init()
    .unwrap();

  let bind_address = format!("{}:{}", cli.host, cli.port);

  log::info!(
    "{} {} listening on {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    bind_address
  );
  HttpServer::new(|| {
    let auth = HttpAuthentication::bearer(bearer_validator);
    App::new()
      .app_data(AuthApi::default())
      .data(Api::default())
      .wrap(middleware::Logger::default())
      .wrap(auth)
      .service(
        web::scope("/api/v1")
          .route(
            "/quotes/{tickers}/{period}",
            web::get().to(handlers::tickers),
          )
          .route("/about", web::get().to(handlers::about)),
      )
  })
  .bind(bind_address)?
  .run()
  .await
}
