# Endpoint API

## `/api`

- POST `/login`

### requset

```JSON
{
    "email": "abc@def.xy",
    "password": "P@ssword!"
}
```

### response

```JSON
{
    "token": "some.toke.here",
}
```

## `/api/v1`

- GET `/quotes/{tickers}/{period}`

```JSON
  {
    "eth-usd": {
        "meta": {
            "currency": "USD",
            "symbol": "ETH-USD",
            "exchange_name": "CCC",
            "instrument_type": "CRYPTOCURRENCY",
            "first_trade_date": 1438902000,
            "regular_market_time": 1606928822,
            "gmtoffset": 0,
            "timezone": "GMT",
            "exchange_timezone_name": "Europe/London",
            "regular_market_price": 587.0153,
            "chart_previous_close": 149.0592,
            "previous_close": null,
            "scale": null,
            "price_hint": 2,
            "data_granularity": "1d",
            "range": "1y",
            "valid_ranges": [
                "1d",
                "5d",
                "1mo",
                "3mo",
                "6mo",
                "1y",
                "2y",
                "5y",
                "10y",
                "ytd",
                "max"
            ]
        },
        "quotes": [
            {
                "timestamp": "2019-12-02T00:00:00+00:00",
                "open": 151.17547607421875,
                "high": 152.11695861816406,
                "low": 147.60679626464844,
                "volume": 6670561362,
                "close": 149.0591583251953,
                "adjclose": 149.0591583251953
            },
        ]
    }
  }
```

- GET `/search?term={term}`

```JSON
[
    {
        "exchange": "NMS",
        "short_name": "Tesla, Inc.",
        "quote_type": "EQUITY",
        "symbol": "TSLA",
        "index": "quotes",
        "score": 4296619.0,
        "type_display": "Equity",
        "long_name": "Tesla, Inc."
    }, ...
```
