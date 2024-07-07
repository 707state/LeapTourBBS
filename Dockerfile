# Rust
FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR ./
COPY ./ .
RUN cargo build --target x86_64-unknown-linux-musl --release

# Alpine
FROM alpine:3.17
COPY --from=builder /target/x86_64-unknown-linux-musl/release/LeapTourBBS /
VOLUME ["/data"]
CMD /LeapTourBBS --data-path /data --port 6000 serve

