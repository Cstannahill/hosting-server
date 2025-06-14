FROM rust:1.76 as builder
WORKDIR /usr/src/metrics_exporter
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/metrics_exporter/target/release/metrics_exporter /usr/local/bin/metrics_exporter
COPY config.toml /app/config.toml
CMD ["metrics_exporter"]
