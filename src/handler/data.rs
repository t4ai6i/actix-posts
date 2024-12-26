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
    pub id: i32,

    /// The time at which the message was posted, represented as a string.
    pub posted: String,

    /// The sender of the message (could be a name or an identifier).
    pub sender: String,

    /// The content of the message.
    pub content: String,
}

static DATA_FILENAME: &str = "data.json";

/// Reads a JSON file and deserializes its content into a `Vec<Message>`.
///
/// This function attempts to read the specified file and parse its content as JSON. If the file does not exist,
/// cannot be read, or the JSON is invalid, it will return an empty vector.
///
/// # Arguments
/// - `filename`: A string slice that specifies the name or path of the file to read.
///
/// # Returns
/// A vector of `Message` structs:
/// - If the file is successfully read and the content is valid JSON representing a list of messages, it returns
///   the parsed `Vec<Message>`.
/// - If any error occurs (e.g., file not found, invalid JSON), it returns an empty vector.
///
/// # Behavior
/// - If the file exists but contains invalid JSON, the function will not panic. Instead, it gracefully returns
///   an empty vector.
/// - Uses the `serde_json` crate for JSON deserialization.
///
/// # Example
/// ```rust
/// use crate::actix_posts::handler::data::read_messages_from_file;
/// let messages = read_messages_from_file("data.json");
/// if messages.is_empty() {
///     println!("No messages found or failed to read file.");
/// } else {
///     println!("Found {} messages.", messages.len());
/// }
/// ```
///
/// # Dependencies
/// - Uses the standard library's [`std::fs::read_to_string`] for reading file content.
/// - Requires the `serde_json` crate to deserialize JSON into the `Vec<Message>` type.
pub fn read_messages_from_file(filename: &str) -> Vec<Message> {
    std::fs::read_to_string(filename)
        .ok()
        .and_then(|data| serde_json::from_str::<Vec<Message>>(&data).ok())
        .unwrap_or_default()
}

/// Retrieves all messages from the data file and sorts them by the posted timestamp in descending order.
///
/// This function reads the messages stored in the file specified by `DATA_FILENAME`,
/// deserializes them into a vector of `Message` structs, and then sorts the messages
/// by their `posted` timestamp (most recent messages first).
///
/// # Returns
/// A vector of `Message` structs sorted in descending order by their `posted` timestamp.
///
/// # Behavior
/// - If the data file cannot be read or contains invalid JSON, it will return an empty vector.
/// - Sorting is done in-place on the vector before returning.
///
/// # Example
/// ```rust
/// use crate::actix_posts::handler::data::get_all;
/// let all_messages = get_all();
/// if all_messages.is_empty() {
///     println!("No messages found!");
/// } else {
///     println!("Found {} messages.", all_messages.len());
///     println!("Most recent message: {:?}", all_messages.first());
/// }
/// ```
///
/// # Dependencies
/// - Relies on `read_messages_from_file` to read and deserialize the data.
/// - Relies on `DATA_FILENAME` for the file path.
///
/// # Notes
/// The returned order ensures that the most recent message (based on the `posted` timestamp)
/// is at the beginning of the vector.
pub fn get_all() -> Vec<Message> {
    let mut messages = read_messages_from_file(DATA_FILENAME);
    messages.sort_by(|a, b| b.posted.cmp(&a.posted));
    messages
}

/// Retrieves a single message by its ID.
///
/// This function reads all messages from the file specified by `DATA_FILENAME` and searches
/// for a message with the given `id`. If a matching message is found, it is returned. If no
/// match is found, it returns the default value for `Message`.
///
/// # Arguments
/// - `id`: An integer representing the ID of the message to retrieve.
///
/// # Returns
/// - A `Message` struct with the corresponding ID if found.
/// - Returns the default value of `Message` if no match is found.
///
/// # Behavior
/// - Uses `into_iter()` to consume the vector of messages.
/// - Searches for a `Message` where `id` (converted to `u32`) matches the provided ID using `find`.
/// - Falls back to `Message::default()` if no match is found.
///
/// # Example
/// ```rust
/// use crate::actix_posts::handler::data::get;
/// use crate::actix_posts::handler::data::Message;
/// let message = get(1);
/// if message == Message::default() {
///     println!("Message not found!");
/// } else {
///     println!("Found message: {:?}", message);
/// }
/// ```
///
/// # Dependencies
/// - Relies on `read_messages_from_file` to load and deserialize messages from the file.
/// - Relies on `DATA_FILENAME` for the file path.
/// - Requires `Message` to implement `Default`.
pub fn get(id: i32) -> Message {
    let messages = read_messages_from_file(DATA_FILENAME);
    messages
        .into_iter()
        .find(|m| m.id == id)
        .unwrap_or_default()
}

pub fn create(mut message: Message) -> Message {
    let mut messages = read_messages_from_file(DATA_FILENAME);
    let max = messages.iter().map(|m| m.id).max().unwrap_or_default();
    message.id = max + 1;
    messages.push(message);
    std::fs::write(DATA_FILENAME, serde_json::to_string(&messages).unwrap()).unwrap();
    messages.pop().unwrap()
}

pub fn update(message: &Message) {
    let mut messages = read_messages_from_file(DATA_FILENAME);
    if let Some(index) = messages.iter().position(|m| m.id == message.id) {
        messages[index] = message.clone();
        std::fs::write(DATA_FILENAME, serde_json::to_string(&messages).unwrap()).unwrap();
    }
}

pub fn remove(id: i32) {
    let mut messages = read_messages_from_file(DATA_FILENAME);
    messages.retain(|item| item.id != id);
    std::fs::write(DATA_FILENAME, serde_json::to_string(&messages).unwrap()).unwrap();
}
