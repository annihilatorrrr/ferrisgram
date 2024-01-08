// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::MessageId;
use crate::Bot;

impl Bot {
    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessages, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of MessageId of the sent messages is returned.
    /// <https://core.telegram.org/bots/api#copymessages>
    pub fn copy_messages(
        &self,
        chat_id: i64,
        from_chat_id: i64,
        message_ids: Vec<i64>,
    ) -> CopyMessagesBuilder {
        CopyMessagesBuilder::new(self, chat_id, from_chat_id, message_ids)
    }
}

#[derive(Serialize)]
pub struct CopyMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format @channelusername)
    pub from_chat_id: i64,
    /// Identifiers of 1-100 messages in the chat from_chat_id to copy. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<i64>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to copy the messages without their captions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}

impl<'a> CopyMessagesBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, from_chat_id: i64, message_ids: Vec<i64>) -> Self {
        Self {
            bot,
            chat_id,
            message_thread_id: None,
            from_chat_id,
            message_ids,
            disable_notification: None,
            protect_content: None,
            remove_caption: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn from_chat_id(mut self, from_chat_id: i64) -> Self {
        self.from_chat_id = from_chat_id;
        self
    }

    pub fn message_ids(mut self, message_ids: Vec<i64>) -> Self {
        self.message_ids = message_ids;
        self
    }

    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn remove_caption(mut self, remove_caption: bool) -> Self {
        self.remove_caption = Some(remove_caption);
        self
    }

    pub async fn send(self) -> Result<Vec<MessageId>> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get::<Vec<MessageId>>("copyMessages", Some(&form))
            .await
    }
}