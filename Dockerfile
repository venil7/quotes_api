FROM rust:latest as builder
WORKDIR /usr/src/server
COPY . .
RUN cargo install --path .

FROM ubuntu
RUN apt-get update && apt-get install -y libssl1.1 libssl1.0.0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY ./entry.sh .
CMD ["./entry.sh"]

# FROM rust

# WORKDIR /usr/src/server
# COPY . .
# COPY ./entry.sh /entry.sh

# RUN cargo install --path .

# CMD ["/entry.sh"]