use uuid::Uuid;

use crate::proto::TextMessage;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Debug)]
pub enum Direction {
    Sent,
    Received,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug)]
pub struct Message {
    pub uuid: String,
    pub remote_uuid: String,
    pub direction: Direction,
    pub timestamp: u64,
    pub content: String,
}

impl Message {
    pub fn new(remote_uuid: String, direction: Direction, content: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            remote_uuid,
            direction,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            content,
        }
    }

    pub fn as_proto(&self, service_id: &str) -> TextMessage {
        TextMessage {
            ident: service_id.to_string(),
            timestamp: self.timestamp,
            message: self.content.clone(),
        }
    }
}

impl From<&TextMessage> for Message {
    fn from(value: &TextMessage) -> Self {
        Message::new(value.ident.clone(), Direction::Received, value.message.clone())
    }
}
