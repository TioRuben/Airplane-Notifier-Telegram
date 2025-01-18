# Build stage
FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev \
    && cargo build --release

# Runner stage
FROM debian:bookworm-slim

WORKDIR /app

# Install required libraries
RUN apt-get update && apt-get install -y libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/airplane-notifier /app/airplane-notifier

ENV JSON_HOST="127.0.0.1" \
    JSON_PORT=30047 \
    ALERT_LAT=40.0 \
    ALERT_LON=-3.0 \
    MAX_DISTANCE=10.0 \
    MAX_ALTITUDE=500.0 \
    TELEGRAM_BOT_TOKEN="your-bot-token" \
    TELEGRAM_CHAT_ID="your-chat-id"

CMD ["./airplane-notifier"]
