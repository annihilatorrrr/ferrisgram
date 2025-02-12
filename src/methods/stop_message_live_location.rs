// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::InlineKeyboardMarkup;
use crate::types::Message;
use crate::Bot;

impl Bot {
    /// Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned.
    /// <https://core.telegram.org/bots/api#stopmessagelivelocation>
    pub fn stop_message_live_location(&self) -> StopMessageLiveLocationBuilder {
        StopMessageLiveLocationBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct StopMessageLiveLocationBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the message with live location to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> StopMessageLiveLocationBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            business_connection_id: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    pub fn business_connection_id(mut self, business_connection_id: String) -> Self {
        self.business_connection_id = Some(business_connection_id);
        self
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("stopMessageLiveLocation", Some(&form)).await
    }
}
