use actix_web::ResponseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use yahoo_finance_api as yahoo;

#[derive(Debug)]
pub struct ApiError {}

impl ApiError {
  pub fn new() -> ApiError {
    ApiError {}
  }
}

impl Display for ApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({})", "error")
  }
}

impl Error for ApiError {}

impl From<yahoo::YahooError> for ApiError {
  fn from(_ye: yahoo::YahooError) -> ApiError {
    ApiError::new()
  }
}

impl ResponseError for ApiError {}
