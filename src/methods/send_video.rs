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
    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    /// <https://core.telegram.org/bots/api#sendvideo>
    pub fn send_video<F: InputFile>(&self, chat_id: i64, video: F) -> SendVideoBuilder<F> {
        SendVideoBuilder::new(self, chat_id, video)
    }
}

#[derive(Serialize)]
pub struct SendVideoBuilder<'a, F: InputFile> {
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
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Video caption (may also be used when resending videos by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True if the video needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
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

impl<'a, F: InputFile> SendVideoBuilder<'a, F> {
    pub fn new(bot: &'a Bot, chat_id: i64, video: F) -> Self {
        let mut data = HashMap::new();
        data.insert("video", video);
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
            supports_streaming: None,
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

    pub fn video(mut self, video: F) -> Self {
        self.data.insert("video", video);
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

    pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
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
            .post("sendVideo", Some(&form), Some(self.data))
            .await
    }
}
