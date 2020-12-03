use crate::api::Api;
use crate::domain::period::Period;
use std::convert::TryFrom;

use actix_web::{web, HttpResponse, Result};

pub async fn ticker(
  api: web::Data<Api>,
  web::Path((tickers, period)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
  let period = Period::try_from(period)?;
  let json = api.quotes_for_range(&tickers, period).await?;
  Ok(HttpResponse::Ok().json(json))
}

pub async fn tickers(
  api: web::Data<Api>,
  web::Path((tickers, period)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
  let period = Period::try_from(period)?;
  let map = api.multiple_quotes_for_range(&tickers, period).await?;
  Ok(HttpResponse::Ok().json(map))
}
