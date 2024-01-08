// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns True on success.
    /// <https://core.telegram.org/bots/api#deletemessages>
    pub fn delete_messages(&self, chat_id: i64, message_ids: Vec<i64>) -> DeleteMessagesBuilder {
        DeleteMessagesBuilder::new(self, chat_id, message_ids)
    }
}

#[derive(Serialize)]
pub struct DeleteMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Identifiers of 1-100 messages to delete. See deleteMessage for limitations on which messages can be deleted
    pub message_ids: Vec<i64>,
}

impl<'a> DeleteMessagesBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, message_ids: Vec<i64>) -> Self {
        Self {
            bot,
            chat_id,
            message_ids,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_ids(mut self, message_ids: Vec<i64>) -> Self {
        self.message_ids = message_ids;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("deleteMessages", Some(&form)).await
    }
}