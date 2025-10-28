FROM rust:slim-trixie AS builder
WORKDIR /app

RUN apt update && apt upgrade  -y && apt install build-essential -y

COPY . .
RUN cargo build --release 

FROM debian:trixie-slim

COPY --from=builder /app/target/release/auction_main /usr/local/bin/auction_main
ENTRYPOINT [ "/usr/local/bin/auction_main" ]
