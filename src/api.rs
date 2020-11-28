use crate::domain::Quote;
use crate::error::ApiError;
use chrono::prelude::*;
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;

pub struct Api {
  provider: yahoo::YahooConnector,
}

impl Api {
  pub fn new() -> Api {
    let provider = yahoo::YahooConnector::new();
    Api { provider }
  }

  pub async fn quote_range(&self, ticker: &str, interval: &str) -> Result<Quote, ApiError> {
    let response = self.provider.get_latest_quotes(ticker, interval).await?;
    let quote = response.last_quote()?;
    let time: DateTime<Utc> = DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
    Ok(Quote {
      name: ticker.into(),
      time: format!("{}", time.to_rfc3339()),
      price: quote.close,
    })
  }
}

// pub enum Interval {
//   Minute(u8),
//   Hour(u8),
//   Day(u8),
//   Month(u8),
//   Year(u8),
// }
