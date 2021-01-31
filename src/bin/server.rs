use actix_web::{middleware, web, App, HttpServer};
use log::LevelFilter;
use quotes_api::api::Api;
use quotes_api::auth::bearer_validator;
use quotes_api::auth_api::AuthApi;
use quotes_api::cli;
use quotes_api::handlers;
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
  let jwt_key = cli.jwt_key.clone();

  log::info!(
    "{} {} listening on {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    bind_address
  );
  HttpServer::new(move || {
    let auth = HttpAuthentication::bearer(bearer_validator);
    App::new()
      .app_data(AuthApi::from(&jwt_key))
      .data(Api::default())
      .wrap(middleware::Logger::default())
      .service(
        web::scope("/api")
          .route("/login", web::post().to(handlers::login))
          .route("/about", web::get().to(handlers::about))
          .service(
            web::scope("/v1")
              .wrap(auth)
              .route(
                "/quotes/{tickers}/{period}",
                web::get().to(handlers::tickers),
              )
              .route("/search", web::get().to(handlers::search)),
          ),
      )
  })
  .bind(bind_address)?
  .run()
  .await
}
