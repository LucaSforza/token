FROM rust:slim-trixie AS builder
WORKDIR /app

RUN apt-get update && apt-get upgrade  -y && apt-get install build-essential=12.12 -y --no-install-reccomends

COPY . .
RUN cargo build --release 

FROM debian:trixie-slim

COPY --from=builder /app/target/release/auction_main /usr/local/bin/auction_main
ENTRYPOINT [ "/usr/local/bin/auction_main" ]
