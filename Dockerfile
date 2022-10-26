FROM rust:slim-bullseye AS build

WORKDIR /app
RUN apt-get update
RUN apt-get install -y build-essential clang lld libssl-dev pkg-config
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=build /app/target/release/newsletter newsletter
COPY config config
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter"]