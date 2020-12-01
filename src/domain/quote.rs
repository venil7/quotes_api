use chrono::prelude::*;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
  pub currency: String,
  pub symbol: String,
  pub exchange_name: String,
  pub instrument_type: String,
  pub first_trade_date: i32,
  pub regular_market_time: u32,
  pub gmtoffset: i32,
  pub timezone: String,
  pub exchange_timezone_name: String,
  pub regular_market_price: f64,
  pub chart_previous_close: f64,
  pub previous_close: Option<f64>,
  pub scale: Option<i32>,
  pub price_hint: i32,
  pub data_granularity: String,
  pub range: String,
  pub valid_ranges: Vec<String>,
}

impl From<&yahoo_finance_api::YMetaData> for Meta {
  fn from(m: &yahoo_finance_api::YMetaData) -> Meta {
    Meta {
      scale: m.scale,
      gmtoffset: m.gmtoffset,
      price_hint: m.price_hint,
      range: m.range.to_owned(),
      symbol: m.symbol.to_owned(),
      timezone: m.timezone.to_owned(),
      currency: m.currency.to_owned(),
      previous_close: m.previous_close,
      valid_ranges: m.valid_ranges.clone(),
      first_trade_date: m.first_trade_date,
      exchange_name: m.exchange_name.to_owned(),
      regular_market_time: m.regular_market_time,
      regular_market_price: m.regular_market_price,
      chart_previous_close: m.chart_previous_close,
      instrument_type: m.instrument_type.to_owned(),
      data_granularity: m.data_granularity.to_owned(),
      exchange_timezone_name: m.exchange_timezone_name.to_owned(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerQuotes {
  meta: Meta,
  quotes: Vec<Quote>,
}

impl From<yahoo_finance_api::YResponse> for TickerQuotes {
  fn from(r: yahoo_finance_api::YResponse) -> TickerQuotes {
    let quotes = r
      .quotes()
      .unwrap_or_default()
      .iter()
      .map(|q| Quote::from(q.clone()))
      .collect::<Vec<Quote>>();
    let yquote = r.chart.result.first().unwrap();
    let meta = Meta::from(&yquote.meta);
    TickerQuotes { meta, quotes }
  }
}
