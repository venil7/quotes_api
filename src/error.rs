use actix_web::ResponseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use yahoo_finance_api as yahoo;

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

impl From<&str> for ApiError {
  fn from(s: &str) -> ApiError {
    ApiError {
      message: String::from(s),
    }
  }
}

impl From<String> for ApiError {
  fn from(message: String) -> ApiError {
    ApiError { message }
  }
}

impl From<yahoo::YahooError> for ApiError {
  fn from(ye: yahoo::YahooError) -> ApiError {
    match ye {
      yahoo::YahooError::FetchFailed(code) => ApiError::from(format!("FetchFailed {}", code)),
      yahoo::YahooError::DeserializeFailed(mess) => {
        ApiError::from(format!("DeserializeFailed {}", mess))
      }
      yahoo::YahooError::ConnectionFailed => ApiError::from("ConnectionFailed"),
      yahoo::YahooError::InvalidStatusCode => ApiError::from("InvalidStatusCode"),
      yahoo::YahooError::EmptyDataSet => ApiError::from("EmptyDataSet"),
      yahoo::YahooError::DataInconsistency => ApiError::from("DataInconsistency"),
    }
  }
}

impl From<regex::Error> for ApiError {
  fn from(_re: regex::Error) -> ApiError {
    ApiError::from("regex::Error")
  }
}

impl From<std::num::ParseIntError> for ApiError {
  fn from(_pe: std::num::ParseIntError) -> ApiError {
    ApiError::from("ParseIntError")
  }
}

impl ResponseError for ApiError {}
