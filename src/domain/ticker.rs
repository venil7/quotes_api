use serde::{Deserialize, Serialize};
use yahoo_finance_api as yahoo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
  pub exchange: String,
  pub short_name: String,
  pub quote_type: String,
  pub symbol: String,
  pub index: String,
  pub score: f64,
  pub type_display: String,
  pub long_name: String,
}

impl From<&yahoo::YQuoteItem> for Ticker {
  fn from(qi: &yahoo::YQuoteItem) -> Ticker {
    Ticker {
      exchange: qi.exchange.to_owned(),
      short_name: qi.short_name.to_owned(),
      quote_type: qi.quote_type.to_owned(),
      symbol: qi.symbol.to_owned(),
      index: qi.index.to_owned(),
      score: qi.score,
      type_display: qi.type_display.to_owned(),
      long_name: qi.long_name.to_owned(),
    }
  }
}
