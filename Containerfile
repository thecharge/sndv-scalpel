FROM docker.io/library/rust:1.90-alpine3.22 AS builder

WORKDIR /build

RUN apk add --no-cache build-base musl-dev

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config ./config

RUN cargo build --release --locked --bin scalpel

FROM docker.io/library/alpine:3.22

RUN apk add --no-cache ca-certificates

COPY --from=builder /build/target/release/scalpel /usr/local/bin/scalpel

WORKDIR /workspace
ENTRYPOINT ["/bin/sh"]
