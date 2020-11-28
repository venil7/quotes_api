use actix_web::{middleware, web, App, HttpServer};
use quote_api::api::Api;
use quote_api::auth::bearer_validator;
use quote_api::auth_api::AuthApi;
use quote_api::cli;
use quote_api::handlers;
use structopt::StructOpt;

use actix_web_httpauth::middleware::HttpAuthentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let cli = cli::Cli::from_args();
  let bind_address = format!("{}:{}", cli.host, cli.port);

  println!("Listening on {}", bind_address);
  HttpServer::new(|| {
    let auth = HttpAuthentication::bearer(bearer_validator);
    App::new()
      .app_data(AuthApi::new())
      .data(Api::new())
      .wrap(middleware::Logger::default())
      .wrap(auth)
      .service(
        web::scope("/api/v1").route("/quote/{ticker}/{period}", web::get().to(handlers::quote)),
      )
  })
  .bind(bind_address)?
  .run()
  .await
}
