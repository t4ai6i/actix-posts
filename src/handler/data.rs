use serde::{Deserialize, Serialize};

static DATA_FILENAME: &str = "data.json";

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

/// Adds a new message to the storage with a unique ID.
///
/// This function handles the creation of a new `Message` by reading the existing messages from
/// a JSON file, determining the current highest ID, assigning a new unique ID to the provided
/// message, and saving the updated message list back to the file.
///
/// # Arguments
///
/// * `message` - A mutable `Message` object that contains the data for the new message. The `id`
/// will be overridden and assigned a unique value.
///
/// # Returns
///
/// * `Message` - The newly created message, including its assigned unique ID.
///
/// # Behavior
///
/// 1. Reads the current list of messages from the file specified by `DATA_FILENAME`.
/// 2. Finds the highest existing message ID in the list.
/// 3. Sets the new message's `id` to one higher than the current maximum ID or `1` if the list is empty.
/// 4. Writes the updated list of messages (including the new message) back to the file.
/// 5. Returns the newly added message.
///
/// # Panics
///
/// This function will panic if any of the following occur:
/// - The data file (`DATA_FILENAME`) cannot be read or written.
/// - Serialization or deserialization of the message list fails.
///
/// # Notes
///
/// This function assumes that `read_messages_from_file` and `serde_json` are used correctly and are
/// compatible with the `Message` structure. The file specified by `DATA_FILENAME` must be accessible
/// and valid in JSON format.
pub fn create(mut message: Message) -> Message {
    let mut messages = read_messages_from_file(DATA_FILENAME);
    let max = messages.iter().map(|m| m.id).max().unwrap_or_default();
    message.id = max + 1;
    messages.push(message);
    std::fs::write(DATA_FILENAME, serde_json::to_string(&messages).unwrap()).unwrap();
    messages.pop().unwrap()
}

/// Updates an existing message in the storage.
///
/// This function takes a reference to a `Message` and updates the corresponding entry in the
/// list of stored messages. The update is performed by matching the `id` of the provided message
/// with an existing entry in the list.
///
/// # Arguments
///
/// * `message` - A reference to a `Message` object containing the updated data. The object must
/// have an `id` field that matches an existing message in the storage.
///
/// # Behavior
///
/// 1. Reads the current list of messages stored in the file specified by `DATA_FILENAME`.
/// 2. Searches for a message with the same `id` as the provided one.
/// 3. If a match is found, replaces the existing message with the provided one.
/// 4. Writes the updated list of messages back to the file.
///
/// If no message with the same `id` exists, the function performs no updates and no errors are raised.
///
/// # Panics
///
/// This function will panic if any of the following occur:
/// - The data file (`DATA_FILENAME`) cannot be read or written.
/// - Serialization or deserialization of the message list fails.
///
/// # Limitations
///
/// This function assumes that:
/// - The file specified by `DATA_FILENAME` exists, is accessible, and is properly formatted.
/// - The list of messages fits in memory since it loads the entire file contents at once.
///
/// In production scenarios, improved error handling and support for larger datasets may be necessary.
pub fn update(message: &Message) {
    let mut messages = read_messages_from_file(DATA_FILENAME);
    if let Some(index) = messages.iter().position(|m| m.id == message.id) {
        messages[index] = message.clone();
        std::fs::write(DATA_FILENAME, serde_json::to_string(&messages).unwrap()).unwrap();
    }
}

/// Removes a message from the storage based on its ID.
///
/// This function deletes a message from the list of stored messages by matching the provided `id`.
/// After removing the message, it updates the storage file with the modified list.
///
/// # Arguments
///
/// * `id` - An `i32` representing the ID of the message to be removed.
///
/// # Behavior
///
/// 1. Reads the current list of messages stored in the file specified by `DATA_FILENAME`.
/// 2. Filters out the message with the specified `id` using the `retain` method.
/// 3. Writes the updated list of messages back to the file.
///
/// If no message with the provided `id` exists, the function silently proceeds without making changes.
///
/// # Panics
///
/// This function will panic if any of the following occur:
/// - The data file (`DATA_FILENAME`) cannot be read or written.
/// - Serialization or deserialization of the message list fails.
///
/// # Limitations
///
/// - The function assumes that the file specified by `DATA_FILENAME` exists, is accessible, and
///   contains valid JSON-formatted data.
/// - Removes messages entirely based on the `id` field. Make sure the `id` is accurate.
///
/// # Notes
///
/// This function uses `retain` to filter out messages, which is efficient for small to moderately sized datasets.
/// For larger datasets, a more scalable solution may need to be considered.
pub fn remove(id: i32) {
    let mut messages = read_messages_from_file(DATA_FILENAME);
    messages.retain(|item| item.id != id);
    std::fs::write(DATA_FILENAME, serde_json::to_string(&messages).unwrap()).unwrap();
}
