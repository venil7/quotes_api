use crate::api::Api;
use crate::domain::Period;
use std::convert::TryFrom;

use actix_web::{web, HttpResponse, Result};

pub async fn period(
  api: web::Data<Api>,
  web::Path((tickers, period)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
  let period = Period::try_from(period)?;
  let json = api.quotes_for_range(&tickers, period).await?;
  Ok(HttpResponse::Ok().json(json))
}

pub async fn latest(
  api: web::Data<Api>,
  web::Path(tickers): web::Path<String>,
) -> Result<HttpResponse> {
  let json = api.latest(&tickers).await?;
  Ok(HttpResponse::Ok().json(json))
}
