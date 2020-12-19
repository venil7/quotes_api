use crate::domain::ticker::Ticker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
  tickers: Vec<Ticker>,
}

impl SearchResult {
  pub fn new(tickers: Vec<Ticker>) -> SearchResult {
    SearchResult { tickers }
  }
}
