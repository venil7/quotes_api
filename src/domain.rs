use crate::error::ApiError;
use chrono::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::num::ParseIntError;
use std::time::{Duration, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Quote {
  pub timestamp: String,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub volume: u64,
  pub close: f64,
  pub adjclose: f64,
}

impl From<yahoo_finance_api::Quote> for Quote {
  fn from(q: yahoo_finance_api::Quote) -> Quote {
    let timestamp: DateTime<Utc> = DateTime::from(UNIX_EPOCH + Duration::from_secs(q.timestamp));
    Quote {
      timestamp: timestamp.to_rfc3339(),
      open: q.open,
      high: q.high,
      low: q.low,
      volume: q.volume,
      close: q.close,
      adjclose: q.adjclose,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Period {
  Minute(u8),
  Hour(u8),
  Day(u8),
  Month(u8),
  Year(u8),
  Unknown,
}

impl Period {
  pub fn to_yahoo_range(&self) -> Result<String, ApiError> {
    match self {
      Period::Minute(num_hours) => Ok(format!("{}m", num_hours)),
      Period::Hour(num_hours) => Ok(format!("{}ho", num_hours)),
      Period::Day(num_days) => Ok(format!("{}d", num_days)),
      Period::Month(num_months) => Ok(format!("{}mo", num_months)),
      Period::Year(num_years) => Ok(format!("{}y", num_years)),
      _ => Err(ApiError::default()),
    }
  }

  pub fn to_matching_yahoo_granularity(&self) -> Result<String, ApiError> {
    match self {
      Period::Minute(_num_minutes) => Period::Minute(1).to_yahoo_range(),
      Period::Hour(_num_hours) => Period::Minute(1).to_yahoo_range(),
      Period::Day(_num_days) => Period::Minute(15).to_yahoo_range(),
      Period::Month(_num_months) => Period::Day(1).to_yahoo_range(),
      Period::Year(_num_years) => Period::Day(1).to_yahoo_range(),
      _ => Err(ApiError::default()),
    }
  }
}

impl From<String> for Period {
  fn from(s: String) -> Period {
    let re = Regex::new(r"^(\d)([A-z]{1,2})$").unwrap();
    match re.captures(&s) {
      Some(m) => {
        let digit_part: Result<u8, ParseIntError> = m[1].to_owned().parse();
        match digit_part {
          Ok(digit) if digit > 0 => {
            let alpha_part = m[2].to_owned();
            match &alpha_part[..] {
              "m" => Period::Minute(digit),
              "h" => Period::Hour(digit),
              "d" => Period::Day(digit),
              "M" => Period::Month(digit),
              "y" => Period::Year(digit),
              _ => Period::Unknown,
            }
          }
          _ => Period::Unknown,
        }
      }
      None => Period::Unknown,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn period_from_string_minute() {
    let str1 = String::from("6m");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Minute(6));
  }

  #[test]
  fn period_from_string_day() {
    let str1 = String::from("1d");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Day(1));
  }

  #[test]
  fn period_from_string_month() {
    let str1 = String::from("6M");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Month(6));
  }
  #[test]
  fn period_from_string_year() {
    let str1 = String::from("1y");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Year(1));
  }

  #[test]
  fn period_from_string_fail_neg() {
    let str1 = String::from("0mo");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Unknown);
  }
}
