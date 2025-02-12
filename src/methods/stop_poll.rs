// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::InlineKeyboardMarkup;
use crate::types::Poll;
use crate::Bot;

impl Bot {
    /// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
    /// <https://core.telegram.org/bots/api#stoppoll>
    pub fn stop_poll(&self, chat_id: i64, message_id: i64) -> StopPollBuilder {
        StopPollBuilder::new(self, chat_id, message_id)
    }
}

#[derive(Serialize)]
pub struct StopPollBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    /// A JSON-serialized object for a new message inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> StopPollBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, message_id: i64) -> Self {
        Self {
            bot,
            business_connection_id: None,
            chat_id,
            message_id,
            reply_markup: None,
        }
    }

    pub fn business_connection_id(mut self, business_connection_id: String) -> Self {
        self.business_connection_id = Some(business_connection_id);
        self
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }

    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub async fn send(self) -> Result<Poll> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("stopPoll", Some(&form)).await
    }
}
