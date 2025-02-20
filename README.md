# Airplane Notifier

A Rust application that monitors aircraft in your vicinity using ADS-B data and sends notifications via Telegram when aircraft are detected within specified parameters.

## Features

- Connects to a JSON format ADS-B data source (such as dump1090)
- Monitors aircraft within a configurable radius and altitude
- Sends notifications via Telegram when aircraft enter the monitored zone
- Automatically reconnects if connection is lost

## Prerequisites

- ADS-B receiver outputting JSON format data (like dump1090)
- Telegram bot token and chat ID


## Configuration

Create a `.env` file in the project root with the following variables:

```env
JSON_HOST=localhost
JSON_PORT=30005
HOME_LAT=51.5074
HOME_LON=-0.1278
MAX_DISTANCE_KM=10
MAX_ALTITUDE_FEET=5000
TELEGRAM_BOT_TOKEN=your_bot_token
TELEGRAM_CHAT_ID=your_chat_id
```

## Running with Docker

You can pull the image either from GHCR or Docker Hub

```
docker pull ghcr.io/tioruben/airplane-notifier:latest
```

```
docker pull tioruben/airplane-notifier:latest
```

Download the [.env.example](.env.example) and rename to .env Edit to fit your configuration

Run the docker image using the `.env` file:

```
docker run --env-file .env ghcr.io/tioruben/airplane-notifier:latest
```

```
docker run --env-file .env tioruben/airplane-notifier:latest
```

## Building

Rust toolchain for your arch is needed

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## How it Works

1. Connects to an ADS-B receiver using the JSON protocol from dump1090
2. Decodes aircraft position messages
3. Calculates distance from your location using the Haversine formula
4. Sends Telegram notifications when aircraft enter your monitored zone
5. Cleans up stale aircraft data automatically

## License

MIT License
