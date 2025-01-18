mod json_decoder;
use crate::json_decoder::JsonDecoder;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;

#[derive(Debug)]
struct Aircraft {
    last_seen: u64,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    notified: bool,
}

struct Config {
    home_lat: f64,
    home_lon: f64,
    max_distance: f64,
    max_altitude: f64,
    telegram_token: String,
    telegram_chat_id: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let config = Config {
        home_lat: env::var("HOME_LAT").unwrap().parse().unwrap(), // 51.5074, // Replace with your latitude
        home_lon: env::var("HOME_LON").unwrap().parse().unwrap(), // -0.1278, // Replace with your longitude
        max_distance: env::var("MAX_DISTANCE_KM").unwrap().parse().unwrap(),
        max_altitude: env::var("MAX_ALTITUDE_FEET").unwrap().parse().unwrap(),
        telegram_token: env::var("TELEGRAM_BOT_TOKEN").unwrap(),
        telegram_chat_id: env::var("TELEGRAM_CHAT_ID").unwrap().as_str().to_string(),
    };

    println!("Starting Airplane Notifier...");

    // Send initialization message
    // if let Err(e) = send_telegram_notification(
    //     &config.telegram_token,
    //     &config.telegram_chat_id,
    //     format!(
    //         "🛩 Airplane Notifier started! Monitoring area within {}km and below {}ft",
    //         config.max_distance, config.max_altitude
    //     ),
    // ).await {
    //     println!("Failed to send init message: {}", e);
    // }

    println!("Monitoring area within {}km and below {}ft", config.max_distance, config.max_altitude);

    let mut aircraft_map: HashMap<String, Aircraft> = HashMap::new();
    let host = env::var("JSON_HOST").unwrap();
    let port = env::var("JSON_PORT").unwrap();

    loop {
        match connect_and_process(&config, &mut aircraft_map, &host, &port).await {
            Ok(_) => println!("Connection closed, reconnecting..."),
            Err(e) => println!("Error: {}, reconnecting...", e),
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn connect_and_process(
    config: &Config,
    aircraft_map: &mut HashMap<String, Aircraft>,
    host: &str,
    port: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
    let mut decoder = JsonDecoder::new(stream);

    while let Ok(data) = decoder.next().await {

        if let (Some(lat), Some(lon), Some(alt), Some(desc), Some(registration), Some(aircraft_type), Some(flight_id)) = (data.lat, data.lon, data.alt_baro, data.desc, data.r, data.t, data.flight) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let hex_code = data.hex.clone();
            let aircraft = aircraft_map.entry(hex_code).or_insert(Aircraft {
                last_seen: now,
                latitude: lat,
                longitude: lon,
                altitude: alt,
                notified: false,
            });
            
            aircraft.last_seen = now;
            aircraft.latitude = lat;
            aircraft.longitude = lon;
            aircraft.altitude = alt;
            check_and_notify(config, &data.hex, aircraft, desc, aircraft_type, registration, flight_id).await?;
        }

        // Clean up old aircraft
        aircraft_map.retain(|_, v| v.last_seen + 60 > SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs());
    }
    Ok(())
}

async fn check_and_notify(
    config: &Config,
    icao: &str,
    aircraft: &mut Aircraft,
    desc: String,
    aircraft_type: String,
    registration: String,  
    flight_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = haversine::Location {
        latitude: config.home_lat,
        longitude: config.home_lon,
    };

    let end = haversine::Location {
        latitude: aircraft.latitude,
        longitude: aircraft.longitude,
    };
    let distance = haversine::distance(
        start,
        end,
        haversine::Units::Kilometers,
    );

    println!(
        "Aircraft: {} ({}) at {:.1}km, {:.0}ft",
        desc, aircraft_type, distance, aircraft.altitude
    );

    if distance <= config.max_distance && aircraft.altitude <= config.max_altitude && !aircraft.notified {
        aircraft.notified = true;
        
        send_telegram_notification(
            &config.telegram_token,
            &config.telegram_chat_id,
            format!(
                "✈️ Aircraft detected!\nHex: {}\nType: {} ({})\nRegistration: {}\nDistance: {:.1}km\nAltitude: {:.0}ft\n\nhttps://www.flightradar24.com/{}",
                icao, desc, aircraft_type, registration, distance, aircraft.altitude, flight_id
            ),
        )
        .await?;
    } else if distance > config.max_distance || aircraft.altitude > config.max_altitude {
        aircraft.notified = false;
    }

    Ok(())
}

async fn send_telegram_notification(
    token: &str,
    chat_id: &str,
    message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        token
    );

    println!("Sending Telegram notification: {} to url {} with chat id {}", message, url, chat_id);

    let client = reqwest::Client::new();
    let request = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": message,
        }))
        .build();

    match request {
        Ok(req) => {
            // Inspect the request
            println!("{:?}", req);
    
            // Send the request
            let response = client.execute(req).await?;
            println!("{:?}", response);
            println!("{:?}", response.text().await?);
            // Handle the response...
        }
        Err(e) => {
            // Handle the error
            eprintln!("Error building the request: {:?}", e);
        }
    }

    // let request = reqwest::Client::new()
    //     .post(&url)
    //     .json(&serde_json::json!({
    //         "chat_id": chat_id,
    //         "text": message,
    //     }))
    //     .send()
    //     .await?;

    Ok(())
}
