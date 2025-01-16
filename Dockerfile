FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

ENV BEAST_HOST="127.0.0.1" \
    BEAST_PORT=30005 \
    ALERT_LAT=40.0 \
    ALERT_LON=-3.0 \
    MAX_DISTANCE=10.0 \
    MAX_ALTITUDE=500.0 \
    TELEGRAM_BOT_TOKEN="your-bot-token" \
    TELEGRAM_CHAT_ID="your-chat-id"

CMD ["./target/release/airplane-notifier"]