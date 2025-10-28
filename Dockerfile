# 1. Builder Stage
FROM rust:slim-trixie AS builder
WORKDIR /app

# 'build-base' is Alpine's equivalent of 'build-essential'
RUN apt update && apt upgrade  -y && apt install build-essential -y

# 2. Cache dependencies
# Copy only the manifests
COPY Cargo.toml Cargo.lock ./
COPY out/bindings/  ./out/bindings/

# Create dummy project to build *only* dependencies
RUN echo "fn main() {}" > main.rs
RUN cargo build --release 

# 3. Copy source code and build final binary
# This overwrites the dummy main.rs
COPY . .
# This build will be fast, using cached layers
RUN  --mount=type=cache,target=/app/target \
  --mount=type=cache,target=/usr/local/cargo/registry \
  cargo build --release 

# ---

# 2. Runner Stage
FROM debian:trixie


# Copy the musl-compiled binary from the builder
COPY --from=builder /app/target/release/auction_main /usr/local/bin/auction_main
ENTRYPOINT [ "/usr/local/bin/auction_main" ]
