# Builder
FROM rust:1.63.0-slim-bullseye AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
        && apt-get install -y --no-install-recommends openssl ca-certificates \
        && apt-get autoremove -y \
        && apt-get clean -y \
        && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/newsletter newsletter
COPY config config
ENV APP_ENVIRONMENT production
ENV RUST_LOG trace
ENTRYPOINT ["./newsletter"]

