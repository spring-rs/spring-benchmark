# rust builder
FROM rust:alpine AS builder

WORKDIR /build

COPY Cargo.toml ./
RUN cargo fetch

COPY . .

RUN cargo build --release

# runner container
FROM alpine:latest

ENV RUST_LOG=info
ENV APP_USER=rust

RUN useradd -m ${APP_USER}

WORKDIR /app

COPY --from=builder /build/target/release/spring-rs .

RUN chown -R ${APP_USER}:${APP_USER} /app/spring-rs

USER ${APP_USER}

CMD ["./spring-rs"]