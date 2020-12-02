FROM rust:latest as builder
WORKDIR /usr/src/server
COPY . .
RUN cargo install --path .

FROM ubuntu
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY ./entry.sh .
CMD ["./entry.sh"]