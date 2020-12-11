## Status

![Rust](https://github.com/venil7/quotes_api/workflows/Rust/badge.svg)

## [API](./API.md)

## Todo

- [ ] User management
- [x] Authentication with JWT
- [ ] Redis caching layer
- [x] CI/CD pipeline
- [x] Docker image on hub

### Docker

```bash
$ docker run \
  --rm \
  -p 8088:8088 \
  -e HOST=127.0.0.1 \
  -e PORT=8088 \
  -e LOGLEVEL=info \
  -e JWTKEY=$SECRET
  quotes_api:latest
```

### Docker compose

```yml
version: "3"

services:
  pihole:
    container_name: quotes_api
    image: darkruby/quotes_api:latest
    ports:
      - "8088:8088/tcp"
    environment:
      HOST: "0.0.0.0"
      PORT: "8088"
      JWTKEY: "S3cret"
      LOGLEVEL: "info"
    restart: unless-stopped
```
