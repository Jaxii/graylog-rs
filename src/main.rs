use std::{
    io::Write,
    net::TcpStream,
    time::{SystemTime, UNIX_EPOCH},
    error::Error,
};
use serde::{Serialize, Deserialize};
use serde_json::json;

// Struct representing a log message.
#[derive(Serialize, Deserialize)]
struct LogMessage {
    message: String,
    timestamp: u64,
    level: String,
}

// Function to send a log message to the graylog server.
fn send_log_message(log_message: &LogMessage) -> Result<(), Box<dyn Error>> {
    // Establish a connection to the graylog server.
    let mut stream = TcpStream::connect("graylog.example.com:12201")?;

    // Serialize the log message as JSON.
    let json_string = serde_json::to_string(log_message)?;

    // Write the serialized JSON to the stream.
    stream.write_all(json_string.as_bytes())?;
    Ok(())
}

// Function to create and send a log message with the specified message and level.
fn log(message: &str, level: &str) -> Result<(), Box<dyn Error>> {
    // Create a new log message with the current timestamp.
    let log_message = LogMessage {
        message: message.to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_secs(),
        level: level.to_string(),
    };

    // Send the log message to the graylog server.
    send_log_message(&log_message)
}

// Example usage of the logger.
fn main() -> Result<(), Box<dyn Error>> {
    log("Hello, world!", "info")?;
    
    Ok(())
}
