// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::input_file::InputFile;
use crate::types::Message;
use crate::types::{InlineKeyboardMarkup, MessageEntity, ReplyParameters};
use crate::Bot;
use std::collections::HashMap;

impl Bot {
    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    /// <https://core.telegram.org/bots/api#sendanimation>
    pub fn send_animation<F: InputFile>(
        &self,
        chat_id: i64,
        animation: F,
    ) -> SendAnimationBuilder<F> {
        SendAnimationBuilder::new(self, chat_id, animation)
    }
}

#[derive(Serialize)]
pub struct SendAnimationBuilder<'a, F: InputFile> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip)]
    data: HashMap<&'a str, F>,
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Animation caption (may also be used when resending animation by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True if the animation needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a, F: InputFile> SendAnimationBuilder<'a, F> {
    pub fn new(bot: &'a Bot, chat_id: i64, animation: F) -> Self {
        let mut data = HashMap::new();
        data.insert("animation", animation);
        Self {
            bot,
            data,
            business_connection_id: None,
            chat_id,
            message_thread_id: None,
            duration: None,
            width: None,
            height: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
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

    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn animation(mut self, animation: F) -> Self {
        self.data.insert("animation", animation);
        self
    }

    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn thumbnail(mut self, thumbnail: F) -> Self {
        self.data.insert("thumbnail", thumbnail);
        self
    }

    pub fn caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn parse_mode(mut self, parse_mode: String) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
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

    pub fn reply_parameters(mut self, reply_parameters: ReplyParameters) -> Self {
        self.reply_parameters = Some(reply_parameters);
        self
    }

    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .post("sendAnimation", Some(&form), Some(self.data))
            .await
    }
}
