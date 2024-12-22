use serde::{Deserialize, Serialize};

/// Represents a user message.
///
/// Each `Message` instance contains details about a message, such as
/// its unique identifier, the time it was posted, the sender, and the content.
///
/// # Fields
/// - `id`: A unique identifier for the message.
/// - `posted`: A timestamp indicating when the message was posted, stored as a string.
/// - `sender`: The name or identifier of the sender of the message.
/// - `content`: The content of the message, stored as a string.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Message {
    /// Unique identifier for the message.
    pub id: u32,

    /// The time at which the message was posted, represented as a string.
    pub posted: String,

    /// The sender of the message (could be a name or an identifier).
    pub sender: String,

    /// The content of the message.
    pub content: String,
}

static DATA_FILENAME: &str = "data.json";

pub fn get_all() -> Vec<Message> {
    let data = std::fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json: Vec<Message> = serde_json::from_str(&data).unwrap();
    json.sort_by(|a, b| b.posted.cmp(&a.posted));
    json
}
