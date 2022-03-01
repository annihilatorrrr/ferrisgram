// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// This object represents a service message about a voice chat scheduled in the chat.
/// <https://core.telegram.org/bots/api#voicechatscheduled>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceChatScheduled {
    /// Point in time (Unix timestamp) when the voice chat is supposed to be started by a chat administrator
    pub start_date: i64,
}