use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Local};

#[derive(Serialize, Deserialize)]
pub struct Message {
    sender_name: String,
    pub text: String,
    is_service: bool,
    timestamp: DateTime<Local>
}

impl Message {
    pub fn new(sender_name: String, text: String, is_service: bool) -> Self {
        let timestamp: DateTime<Local> = Local::now();
        return Self {
            sender_name,
            text,
            is_service,
            timestamp
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}][{}]: {}", self.timestamp, self.sender_name, self.text)
    }
}

impl From<&str> for Message {
    fn from(data: &str) -> Self {
        return serde_json::from_str(data).expect("Error while deserializing");
    }
}

impl From<&String> for Message {
    fn from(data: &String) -> Self {
        return serde_json::from_str(&data).expect("Error while deserializing");
    }
}

impl From<&Message> for String {
    fn from(data: &Message) -> Self {
        return serde_json::to_string(&data).expect("Error while serializing");
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self {
            sender_name: self.sender_name.clone(),
            text: self.text.clone(),
            is_service: self.is_service,
            timestamp: self.timestamp
        }
    }
}