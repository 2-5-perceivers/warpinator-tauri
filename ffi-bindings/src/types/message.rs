#[derive(uniffi::Enum)]
pub enum Direction {
    Sent,
    Received,
}

#[derive(uniffi::Record)]
pub struct Message {
    pub uuid: String,
    pub remote_uuid: String,
    pub direction: Direction,
    pub timestamp: u64,
    pub content: String,
}

impl From<&warpinator_lib::types::message::Message> for Message {
    fn from(value: &warpinator_lib::types::message::Message) -> Self {
        Self {
            uuid: value.uuid.clone(),
            remote_uuid: value.remote_uuid.clone(),
            direction: match value.direction {
                warpinator_lib::types::message::Direction::Sent => Direction::Sent,
                warpinator_lib::types::message::Direction::Received => Direction::Received,
            },
            timestamp: value.timestamp,
            content: value.content.clone(),
        }
    }
}
