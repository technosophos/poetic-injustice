FROM rust:latest AS builder
WORKDIR /build
COPY . .
RUN cargo install --path . && cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /build/release/poetic-injustice .
CMD ["/app/poetic-injustice"]