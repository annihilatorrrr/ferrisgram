// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::MessageEntity;
use serde::{Deserialize, Serialize};

/// This object contains information about one answer option in a poll to send.
/// <https://core.telegram.org/bots/api#inputpolloption>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputPollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Optional. Mode for parsing entities in the text. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// Optional. A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of text_parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}
