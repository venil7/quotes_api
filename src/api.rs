use crate::database;
use crate::domain::period::Period;
use crate::domain::quote::{QuotesResult, TickerQuotes};
use crate::domain::search_result::SearchResult;
use crate::domain::ticker::Ticker;
use crate::error::ApiError;
use std::collections::HashMap;
use yahoo_finance_api as yahoo;

pub struct Api {
  provider: yahoo::YahooConnector,
  database: database::Database,
}

impl Api {
  pub fn try_new(database_url: &str) -> Result<Self, ApiError> {
    let provider = yahoo::YahooConnector::new();
    let database = database::Database::try_new(database_url)?;
    Ok(Api { provider, database })
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
      .await
      .map_err(|e| format!("get_quote_range {}", e))?;

    Ok(response.into())
  }

  pub async fn multiple_quotes_for_range(
    &self,
    tickers: &str,
    period: Period,
  ) -> Result<QuotesResult, ApiError> {
    let tickers = tickers.split(',').collect::<Vec<_>>();
    let mut quotes = HashMap::new();
    let handles = tickers.iter().map(|s| self.quotes_for_range(s, period));
    let results: Vec<Result<_, _>> = futures::future::join_all(handles).await;

    for result in results {
      match result {
        Ok(quote) => {
          quotes.insert(quote.meta.symbol.clone(), quote);
        }
        _ => { /* Errored quote doesn't make it to resulting JSON */ }
      }
    }

    return Ok(QuotesResult::new(quotes));
  }

  pub async fn search(&self, term: &str) -> Result<SearchResult, ApiError> {
    let search_result = self
      .provider
      .search_ticker(term)
      .await
      .map_err(|e| format!("search_ticker {}", e))?;

    let tickers: Vec<Ticker> = search_result.quotes.iter().map(Ticker::from).collect();

    Ok(SearchResult::new(tickers))
  }
}
