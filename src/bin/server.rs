#[macro_use]
extern crate diesel_migrations;
use actix_web::{middleware, web, App, HttpServer};
use log::LevelFilter;
use quotes_api::api::Api;
use quotes_api::auth::bearer_validator;
use quotes_api::auth_api::AuthApi;
use quotes_api::cli;
use quotes_api::database::establish_connection;
use quotes_api::handlers;
use simple_logger::SimpleLogger;
use std::process;
use std::str::FromStr;
use structopt::StructOpt;

use actix_web_httpauth::middleware::HttpAuthentication;

embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let cli = cli::Cli::from_args();

  let conn = establish_connection(&cli.database_url).unwrap();
  if let Err(migrations_error) = embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
  {
    println!("Migration error: {}", migrations_error);
    process::exit(1);
  }

  SimpleLogger::new()
    .with_level(LevelFilter::from_str(&cli.log_level).unwrap_or(LevelFilter::Info))
    .init()
    .unwrap();

  let bind_address = format!("{}:{}", cli.host, cli.port);
  let jwt_key = cli.jwt_key.clone();
  let database_url = cli.database_url.clone();

  log::info!(
    "{} {} listening on {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    bind_address
  );
  HttpServer::new(move || {
    let auth = HttpAuthentication::bearer(bearer_validator);
    let api = Api::try_new(&database_url).unwrap();
    App::new()
      .app_data(AuthApi::from(&jwt_key))
      .data(api)
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
