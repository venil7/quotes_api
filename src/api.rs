use crate::domain::Period;
use crate::domain::Quote;
use crate::error::ApiError;
use log;
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
  ) -> Result<Vec<Quote>, ApiError> {
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
    let quotes = response
      .quotes()?
      .iter()
      .map(|q| Quote::from(q.clone()))
      .collect::<Vec<Quote>>();
    Ok(quotes)
  }
  pub async fn latest(&self, ticker: &str) -> Result<Quote, ApiError> {
    let response = self.provider.get_latest_quotes(ticker, "1d").await?;
    let quote: Quote = response.last_quote()?.into();
    Ok(quote)
  }
}
