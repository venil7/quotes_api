
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
  Hour(u8),
  Day(u8),
  Month(u8),
  Year(u8),
  Unknown,
}

impl From<String> for Period {
  fn from(s: String) -> Period {
    let re = Regex::new(r"^(\d)([A-z]{1,2})$").unwrap();
    match re.captures(&s) {
      Some(m) => {
        let digit_part: Result<u8, ParseIntError> = m[1].to_lowercase().parse();
        match digit_part {
          Ok(digit) if digit > 0 => {
            let alpha_part = m[2].to_lowercase();
            match &alpha_part[..] {
              "h" => Period::Hour(digit),
              "mo" => Period::Month(digit),
              "d" => Period::Day(digit),
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

// impl TryFrom<String> for Period {
//   fn try_from(s: String) -> Result<Period, ApiError> {
//     let p: Period = Period::from(s);
//     match p {
//       Period::Unknown => Err(ApiError::default()),
//       any_other => Ok(any_other),
//     }
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn period_from_string_day() {
    let str1 = String::from("1d");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Day(1));
  }
  #[test]
  fn period_from_string_month() {
    let str1 = String::from("6mo");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Month(6));
  }

  #[test]
  fn period_from_string_fail_neg() {
    let str1 = String::from("0mo");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Unknown);
  }
}
