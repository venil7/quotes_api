use crate::error::ApiError;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Period {
  Days1,
  Days5,
  Months1,
  Months3,
  Months6,
  Years1,
  Years2,
  Years5,
  Year10,
  Ytd,
  Max,
  Unknown,
}

impl Period {
  pub fn to_yahoo_range(&self) -> Result<String, ApiError> {
    match self {
      Period::Days1 => Ok("1d".into()),
      Period::Days5 => Ok("5d".into()),
      Period::Months1 => Ok("1mo".into()),
      Period::Months3 => Ok("3mo".into()),
      Period::Months6 => Ok("6mo".into()),
      Period::Years1 => Ok("1y".into()),
      Period::Years2 => Ok("2y".into()),
      Period::Years5 => Ok("5y".into()),
      Period::Year10 => Ok("10y".into()),
      Period::Ytd => Ok("ytd".into()),
      Period::Max => Ok("max".into()),
      _ => Err(ApiError::from("could not convert to yahoo_range")),
    }
  }

  pub fn to_matching_yahoo_granularity(&self) -> Result<String, ApiError> {
    match self {
      Period::Days1 => Ok("1m".into()),
      Period::Days5 => Ok("15m".into()),
      Period::Months1 | Period::Months3 | Period::Months6 => Ok("1d".into()),
      Period::Years1 | Period::Years2 | Period::Years5 => Ok("1d".into()),
      Period::Year10 => Ok("1mo".into()),
      Period::Ytd => Ok("1d".into()),
      Period::Max => Ok("1d".into()),
      _ => Err(ApiError::from("could not convert to yahoo_granularity")),
    }
  }
}

impl From<String> for Period {
  fn from(s: String) -> Period {
    match s.as_str() {
      "1d" => Period::Days1,
      "5d" => Period::Days5,
      "1mo" => Period::Months1,
      "3mo" => Period::Months3,
      "6mo" => Period::Months6,
      "1y" => Period::Years1,
      "2y" => Period::Years2,
      "5y" => Period::Years5,
      "10y" => Period::Year10,
      "ytd" => Period::Ytd,
      "max" => Period::Max,
      _ => Period::Unknown,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn period_from_string_day() {
    let str1 = String::from("1d");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Days1);
  }

  #[test]
  fn period_from_string_month() {
    let str1 = String::from("6mo");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Months6);
  }
  #[test]
  fn period_from_string_year() {
    let str1 = String::from("1y");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Years1);
  }

  #[test]
  fn period_from_string_fail_neg() {
    let str1 = String::from("0mo");
    let per1 = Period::from(str1);
    assert_eq!(per1, Period::Unknown);
  }
}
