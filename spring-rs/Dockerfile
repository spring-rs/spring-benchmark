# rust builder
FROM rust:alpine AS builder

WORKDIR /build

COPY . .

RUN cargo build --release

# runner container
FROM alpine:latest

ENV RUST_LOG=info

WORKDIR /runner

COPY --from=builder /build/target/release/spring-rs ./app

CMD ["/runner/app"]