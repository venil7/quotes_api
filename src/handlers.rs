use crate::api::Api;
use actix_web::{web, HttpResponse, Result};

pub async fn quote(
  api: web::Data<Api>,
  web::Path((ticker, period)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
  let json = api.quote_range(&ticker, &period).await?;
  Ok(HttpResponse::Ok().json(json))
}
