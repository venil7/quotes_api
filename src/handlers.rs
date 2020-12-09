use crate::api::Api;
use crate::domain::about::About;
use crate::domain::period::Period;
use actix_web::{web, HttpResponse, Result};
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
