FROM rust:1.76 as builder
WORKDIR /usr/src/mongo_backup
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y mongodb-clients && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/mongo_backup/target/release/mongo_backup /usr/local/bin/mongo_backup
CMD ["mongo_backup"]
