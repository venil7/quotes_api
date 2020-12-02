## Status

![Rust](https://github.com/venil7/quotes_api/workflows/Rust/badge.svg)

## [API](./API.md)

## Todo

- [ ] User management
- [ ] Authentication with JWT
- [ ] Redis caching layer
- [ ] CI/CD pipeline
- [ ] Docker image on hub

## Docker

```bash
$ docker build -t quotes_api .
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
