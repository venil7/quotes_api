use crate::domain::period::Period;
use crate::domain::quote::Quote;
use crate::domain::quote::TickerQuotes;
use crate::error::ApiError;
use std::collections::HashMap;
use yahoo_finance_api as yahoo;

pub struct Api {
  provider: yahoo::YahooConnector,
}

impl Default for Api {
  fn default() -> Self {
    let provider = yahoo::YahooConnector::new();
    Api { provider }
  }
}

impl Api {
  pub async fn quotes_for_range(
    &self,
    ticker: &str,
    period: Period,
  ) -> Result<TickerQuotes, ApiError> {
    let granularity = period.to_matching_yahoo_granularity()?;
    let interval = period.to_yahoo_range()?;
    log::info!(
      "requesting Yahoo! for ticker:{}, interval:{}, granularity:{}",
      &ticker,
      &interval,
      &granularity,
    );
    let response = self
      .provider
      .get_quote_range(&ticker, &granularity, &interval)
      .await?;

    Ok(response.into())
  }

  pub async fn multiple_quotes_for_range(
    &self,
    tickers: &str,
    period: Period,
  ) -> Result<HashMap<String, TickerQuotes>, ApiError> {
    let tickers = tickers.split(',').collect::<Vec<_>>();
    let mut map = HashMap::new();
    for ticker in tickers {
      let ticker_quotes = self.quotes_for_range(ticker, period).await?;
      map.insert(ticker.to_owned(), ticker_quotes);
    }

    return Ok(map);
  }

  pub async fn latest(&self, ticker: &str) -> Result<Quote, ApiError> {
    let response = self.provider.get_latest_quotes(ticker, "1d").await?;
    let quote: Quote = response.last_quote()?.into();
    Ok(quote)
  }
}
