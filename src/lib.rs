#![crate_type = "lib"]
use std::{
    io::{Write, self},
    net::{TcpStream, Ipv4Addr},
    time::{SystemTime, UNIX_EPOCH},
    error::Error, fmt::{self, Display},
};

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::json;

struct Connection {
    ip: &'static str, //todo: accept ipv4, ipv6, or web domain
    port: u16,
}

impl Connection {
    //todo
    //default port of 3306
    //ip parameter should be able to take ipv4, ipv6, and web domains
    fn new(ip: &'static str, port: u16) -> Self {
        Connection { ip: ip, port: port }
    }

    fn default() -> Self {
        Connection { ip: "127.0.0.1", port: 3306 }
    }

    fn connect(&self) -> io::Result<TcpStream> {
        TcpStream::connect(self.ip.to_string() + ":" + &self.port.to_string())
    }

}
struct Logger {
    id: u32,
    connection: Option<Connection>,
}

impl Logger {
    fn new(id: u32, connection: Connection) -> Self {
        Logger {
            id: id,
            connection: Some(connection),
        }
    }

    fn with_default_connection(id: u32) -> Self {
        Logger {
            id: id,
            connection: Some(Connection::default()),
        }
    }

    fn log(&self, message: &str, level: Level) -> Result<(), Box<dyn Error>> {
        // Create a new log message with the current timestamp.
        let log_message = LogMessage {
            message: message.to_string(),
            timestamp: Utc::now(),
            level,
        };

        // Send the log message to the graylog server.
        send_log_message(&log_message)
    }
}

// Struct representing a log message.
#[derive(Serialize, Deserialize)]
struct LogMessage {
    timestamp: DateTime<Utc>,
    level: Level,
    message: String,
}

// Enum laying out possible message types
#[derive(Serialize, Deserialize, Clone)]
enum Level {
    INFO,
    WARNING,
    ERROR,
    DEBUG,
    Custom(String),
}

impl Display for Level {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::INFO => write!(f, "INFO"),
            Level::WARNING => write!(f, "WARNING"),
            Level::ERROR => write!(f, "ERROR"),
            Level::DEBUG => write!(f, "DEBUG"),
            Level::Custom(str) => write!(f, "{}", str),
        }
    }

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
fn log(message: &str, level: Level) -> Result<(), Box<dyn Error>> {
    // Create a new log message with the current timestamp.
    let log_message = LogMessage {
        message: message.to_string(),
        timestamp: Utc::now(),
        level,
    };

    // Send the log message to the graylog server.
    send_log_message(&log_message)
}

// // Example usage of the logger.
// fn main() -> Result<(), Box<dyn Error>> {
    
//     log("Hello, world!", Level::INFO)?;
    
//     Ok(())
// }

#[test]
fn test_levels() {

    assert_eq!("INFO", Level::INFO.to_string());
    assert_eq!("WARNING", Level::WARNING.to_string());
    assert_eq!("ERROR", Level::ERROR.to_string());
    assert_eq!("DEBUG", Level::DEBUG.to_string());

}