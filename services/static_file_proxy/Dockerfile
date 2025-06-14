FROM rust:1.76 as builder
WORKDIR /usr/src/static_file_proxy
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/static_file_proxy/target/release/static_file_proxy /usr/local/bin/static_file_proxy
COPY config.toml /app/config.toml
CMD ["static_file_proxy"]
