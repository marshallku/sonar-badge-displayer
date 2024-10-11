FROM rust:1.79-alpine AS base

WORKDIR /usr/src/sonar_badge_displayer

RUN set -eux; \
    apk add --no-cache musl-dev pkgconfig libressl-dev; \
    rm -rf $CARGO_HOME/registry

COPY Cargo.* .

RUN mkdir src && \
    echo 'fn main() {println!("Hello, world!");}' > src/main.rs && \
    cargo build --release && \
    rm target/release/sonar_badge_displayer* && \
    rm target/release/deps/sonar_badge_displayer* && \
    rm -rf src

FROM base AS builder

COPY src src
COPY config.yaml config.yaml
RUN cargo build --release

FROM alpine:3.20.2

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/sonar_badge_displayer/target/release/sonar_badge_displayer .
COPY --from=builder /usr/src/sonar_badge_displayer/config.yaml .

EXPOSE ${PORT}

CMD ["./sonar_badge_displayer"]