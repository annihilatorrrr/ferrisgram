// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::BusinessMessagesDeleted;
use crate::types::Chat;

impl BusinessMessagesDeleted {
    /// This function creates an empty struct for the object BusinessMessagesDeleted.
    pub fn new(business_connection_id: String, chat: Chat, message_ids: Vec<i64>) -> Self {
        Self {
            business_connection_id,
            chat,
            message_ids,
        }
    }
}
