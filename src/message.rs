use std::fmt;
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Local};

#[derive(Serialize, Deserialize)]
pub struct Message {
    sender_name: String,
    text: String,
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

    pub fn get_text(&self) -> String {
        return self.text.to_string();
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}][{}]: {}", self.timestamp, self.sender_name, self.text)
    }
}