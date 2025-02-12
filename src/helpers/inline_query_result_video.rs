// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::InlineQueryResultVideo;

impl InlineQueryResultVideo {
    /// This function creates an empty struct for the object InlineQueryResultVideo.
    pub fn new(
        id: String,
        video_url: String,
        mime_type: String,
        thumbnail_url: String,
        title: String,
    ) -> Self {
        Self {
            id,
            video_url,
            mime_type,
            thumbnail_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
