use actix_web::ResponseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct ApiError {
  message: String,
}

impl Default for ApiError {
  fn default() -> Self {
    ApiError {
      message: String::from(""),
    }
  }
}

impl Display for ApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({})", self.message)
  }
}

impl Error for ApiError {}

impl<T> From<T> for ApiError
where
  T: Into<String>,
{
  fn from(err: T) -> Self {
    ApiError {
      message: err.into(),
    }
  }
}

// impl From<yahoo::YahooError> for ApiError {
//   fn from(ye: yahoo::YahooError) -> ApiError {
//     ApiError::from(format!("{}", ye))
//   }
// }

impl ResponseError for ApiError {}
