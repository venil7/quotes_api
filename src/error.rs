use actix_web::ResponseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use yahoo_finance_api as yahoo;

#[derive(Debug)]
pub struct ApiError {}

impl Default for ApiError {
  fn default() -> Self {
    ApiError {}
  }
}

impl Display for ApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "(erro)")
  }
}

impl Error for ApiError {}

impl From<yahoo::YahooError> for ApiError {
  fn from(_ye: yahoo::YahooError) -> ApiError {
    ApiError::default()
  }
}

impl From<regex::Error> for ApiError {
  fn from(_re: regex::Error) -> ApiError {
    ApiError::default()
  }
}
impl From<std::num::ParseIntError> for ApiError {
  fn from(_pe: std::num::ParseIntError) -> ApiError {
    ApiError::default()
  }
}

impl ResponseError for ApiError {}
