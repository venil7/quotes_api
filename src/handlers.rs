use crate::api::Api;
use crate::auth_api::ApiClaims;
use crate::auth_api::AuthApi;
use crate::domain::about::About;
use crate::domain::login::{Login, LoginToken};
use crate::domain::period::Period;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct TickerSearch {
  ticker: String,
}

pub async fn tickers(
  api: web::Data<Api>,
  web::Path((tickers, period)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
  let period = Period::try_from(period)?;
  let map = api.multiple_quotes_for_range(&tickers, period).await?;
  Ok(HttpResponse::Ok().json(map))
}

pub async fn search(api: web::Data<Api>, search: web::Query<TickerSearch>) -> Result<HttpResponse> {
  let tickers = api.search(&search.ticker).await?;
  Ok(HttpResponse::Ok().json(tickers))
}

pub async fn about() -> Result<HttpResponse> {
  let about = About::default();
  Ok(HttpResponse::Ok().json(about))
}

pub async fn login(req: HttpRequest, credential: web::Json<Login>) -> Result<HttpResponse> {
  if let Some(auth_api) = req.app_data::<AuthApi>() {
    let claims = ApiClaims {
      email: credential.email.clone(),
      write: false,
    };
    let token = auth_api.create_token(&claims)?;
    let login_token = LoginToken { token };
    return Ok(HttpResponse::Ok().json(login_token));
  }

  Ok(HttpResponse::BadRequest().body("Bad data"))
}
