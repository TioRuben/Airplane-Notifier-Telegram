use tokio::io::{AsyncBufReadExt, BufReader, AsyncRead};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AircraftData {
    pub hex: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub alt_baro: Option<f64>,
    pub desc: Option<String>,      
    pub r: Option<String>,
    pub t: Option<String>,
    pub flight: Option<String>,
}

pub struct JsonDecoder<R> {
    reader: BufReader<R>,
}

impl<R: AsyncRead + Unpin> JsonDecoder<R> {
    pub fn new(reader: R) -> Self {
        JsonDecoder {
            reader: BufReader::new(reader),
        }
    }

    pub async fn next(&mut self) -> Result<AircraftData, Box<dyn std::error::Error>> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        if line.is_empty() {
            return Err("Empty line".into());
        }

        let data: AircraftData = serde_json::from_str(&line)?;
        Ok(data)
    }
}
