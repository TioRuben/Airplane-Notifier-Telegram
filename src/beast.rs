use tokio::io::{AsyncRead, AsyncReadExt};
use std::io;

pub struct Beast<R> {
    reader: R,
    buffer: Vec<u8>,
}

#[derive(Debug)]
pub struct ModeSMessage {
    pub message_type: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl ModeSMessage {
    pub fn get_aircraft_position(&self) -> Option<(String, Position)> {
        if self.message_type != 0x1A {  // Only process Extended Squitter messages
            return None;
        }

        // Check if this is an airborne position message (Type Code 9-18)
        let tc = (self.payload[4] >> 3) & 0x1F;
        if !(9..=18).contains(&tc) {
            return None;
        }

        // Extract ICAO address (24 bits)
        let icao = format!("{:02X}{:02X}{:02X}", 
            self.payload[1], 
            self.payload[2], 
            self.payload[3]
        );

        // CPR format - Compact Position Reporting
        let odd_flag = (self.payload[6] & 0x04) != 0;
        let lat_cpr = (((self.payload[6] as u32) & 0x03) << 15) | 
                     ((self.payload[7] as u32) << 7) | 
                     ((self.payload[8] as u32) >> 1);
        let lon_cpr = (((self.payload[8] as u32) & 0x01) << 16) | 
                     ((self.payload[9] as u32) << 8) | 
                     (self.payload[10] as u32);

        // Altitude decoding
        let altitude = match tc {
            9..=18 => {
                let ac = ((self.payload[5] as u16) << 4) | 
                        ((self.payload[6] as u16) >> 4);
                // Q-bit check
                if (ac & 0x10) != 0 {
                    let alt = ((ac & 0x0FE0) >> 1) | (ac & 0x000F);
                    (alt as f64) * 25.0 - 1000.0
                } else {
                    0.0
                }
            },
            _ => 0.0,
        };

        // This is a simplified position calculation
        // In a real implementation, you would need to store both odd and even frames
        // and implement the full CPR algorithm
        let latitude = (lat_cpr as f64) * 360.0 / 131072.0;
        let longitude = (lon_cpr as f64) * 360.0 / 131072.0;

        Some((icao, Position {
            latitude,
            longitude,
            altitude,
        }))
    }
}

impl<R: AsyncRead + Unpin> Beast<R> {
    pub fn new(reader: R) -> Self {
        Beast {
            reader,
            buffer: Vec::with_capacity(1024),
        }
    }

    pub async fn next(&mut self) -> io::Result<ModeSMessage> {
        loop {
            // Look for Beast message start (0x1A)
            while self.buffer.len() < 2 || self.buffer[0] != 0x1A {
                let byte = self.reader.read_u8().await?;
                if byte == 0x1A {
                    self.buffer.clear();
                    self.buffer.push(byte);
                }
            }

            // Read message type
            if self.buffer.len() < 2 {
                self.buffer.push(self.reader.read_u8().await?);
            }

            let msg_type = self.buffer[1];
            let msg_len = match msg_type {
                0x31 => 11,  // Mode S short message
                0x32 => 15,  // Mode S long message
                0x33 => 23,  // Mode S extended message
                _ => {
                    self.buffer.clear();
                    continue;
                }
            };

            // Read full message
            while self.buffer.len() < msg_len {
                let byte = self.reader.read_u8().await?;
                if byte == 0x1A {
                    self.buffer.clear();
                    self.buffer.push(byte);
                    continue;
                }
                self.buffer.push(byte);
            }

            // Process escape sequences
            let mut payload = Vec::with_capacity(msg_len);
            let mut i = 2;
            while i < msg_len {
                if self.buffer[i] == 0x1A {
                    i += 1;
                    payload.push(self.buffer[i] - 0x19);
                } else {
                    payload.push(self.buffer[i]);
                }
                i += 1;
            }

            let message = ModeSMessage {
                message_type: msg_type,
                payload,
            };

            self.buffer.clear();
            return Ok(message);
        }
    }
}
