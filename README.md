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
