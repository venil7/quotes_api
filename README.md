## API

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

## Todo

- [ ] User management
- [ ] Authentication with JWT
- [ ] Redis caching layer
- [ ] CI/CD pipeline
- [ ] Docker image on hub

## Docker

```bash
$ docker build -t quites_api .
```

```bash
$ docker run \
  --rm \
  -p 8333:8088 \
  -e HOST=127.0.0.1 \
  -e PORT=8333 \
  quotes_api:latest \
  server --host $HOST --port $PORT
```
