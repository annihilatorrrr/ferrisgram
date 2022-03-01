// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    /// <https://core.telegram.org/bots/api#pinchatmessage>
    pub fn pin_chat_message(&self, chat_id: i64, message_id: i64) -> PinChatMessageBuilder {
        PinChatMessageBuilder::new(self, chat_id, message_id)
    }
}

#[derive(Serialize)]
pub struct PinChatMessageBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

impl<'a> PinChatMessageBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, message_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            message_id,
            disable_notification: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }

    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("pinChatMessage", Some(&form)).await
    }
}