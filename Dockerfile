FROM rust:latest as builder
WORKDIR /usr/src/server
COPY . .
RUN cargo install --path .

FROM ubuntu
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY ./entry.sh .
CMD ["./entry.sh"]