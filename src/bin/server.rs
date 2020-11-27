use actix_web::{get, App, HttpResponse, HttpServer, Result};
use quote_api::service::Api;

use std::env;

#[get("/")]
async fn index() -> Result<HttpResponse> {
  let api = Api::new();
  let json = api.quote("aapl", "1m").await?;

  Ok(HttpResponse::Ok().json(json))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let bind_address = env::var("BIND_ADDRESS").or::<std::io::Error>(Ok("127.0.0.1:8088".into()))?;

  println!("Listening on {}", bind_address);
  HttpServer::new(|| App::new().service(index))
    .bind(bind_address)?
    .run()
    .await
}
