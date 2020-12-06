use crate::api::Api;
use crate::domain::about::About;
use crate::domain::period::Period;
use std::convert::TryFrom;

use actix_web::{web, HttpResponse, Result};

pub async fn tickers(
  api: web::Data<Api>,
  web::Path((tickers, period)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
  let period = Period::try_from(period)?;
  let map = api.multiple_quotes_for_range(&tickers, period).await?;
  Ok(HttpResponse::Ok().json(map))
}

pub async fn about() -> Result<HttpResponse> {
  let about = About::default();
  Ok(HttpResponse::Ok().json(about))
}
