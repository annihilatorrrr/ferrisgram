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
    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
    /// For sending voice messages, use the sendVoice method instead.
    /// <https://core.telegram.org/bots/api#sendaudio>
    pub fn send_audio<F: InputFile>(&self, chat_id: i64, audio: F) -> SendAudioBuilder<F> {
        SendAudioBuilder::new(self, chat_id, audio)
    }
}

#[derive(Serialize)]
pub struct SendAudioBuilder<'a, F: InputFile> {
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
    /// Audio caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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

impl<'a, F: InputFile> SendAudioBuilder<'a, F> {
    pub fn new(bot: &'a Bot, chat_id: i64, audio: F) -> Self {
        let mut data = HashMap::new();
        data.insert("audio", audio);
        Self {
            bot,
            data,
            business_connection_id: None,
            chat_id,
            message_thread_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
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

    pub fn audio(mut self, audio: F) -> Self {
        self.data.insert("audio", audio);
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

    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn performer(mut self, performer: String) -> Self {
        self.performer = Some(performer);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn thumbnail(mut self, thumbnail: F) -> Self {
        self.data.insert("thumbnail", thumbnail);
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
            .post("sendAudio", Some(&form), Some(self.data))
            .await
    }
}
