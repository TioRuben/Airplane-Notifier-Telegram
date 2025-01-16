# Airplane Notifier

A Rust application that monitors aircraft in your vicinity using ADS-B data and sends notifications via Telegram when aircraft are detected within specified parameters.

## Features

- Connects to a Beast format ADS-B data source
- Monitors aircraft within a configurable radius and altitude
- Sends notifications via Telegram when aircraft enter the monitored zone
- Automatically reconnects if connection is lost

## Prerequisites

- Rust toolchain
- ADS-B receiver outputting Beast format data (like dump1090)
- Telegram bot token and chat ID

## Configuration

Create a `.env` file in the project root with the following variables:

```env
BEAST_HOST=localhost
BEAST_PORT=30005
HOME_LAT=51.5074
HOME_LON=-0.1278
MAX_DISTANCE_KM=10
MAX_ALTITUDE_FEET=5000
TELEGRAM_BOT_TOKEN=your_bot_token
TELEGRAM_CHAT_ID=your_chat_id
```

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## How it Works

1. Connects to an ADS-B receiver using the Beast binary protocol
2. Decodes aircraft position messages
3. Calculates distance from your location using the Haversine formula
4. Sends Telegram notifications when aircraft enter your monitored zone
5. Cleans up stale aircraft data automatically

## License

MIT License
