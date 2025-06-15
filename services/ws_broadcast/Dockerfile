FROM rust:1.76 as builder
WORKDIR /usr/src/ws_broadcast
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/ws_broadcast/target/release/ws_broadcast /usr/local/bin/ws_broadcast
CMD ["ws_broadcast"]
