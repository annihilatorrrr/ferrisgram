// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::InlineQueryResultCachedVideo;

impl InlineQueryResultCachedVideo {
    /// This function creates an empty struct for the object InlineQueryResultCachedVideo.
    pub fn new(id: String, video_file_id: String, title: String) -> Self {
        Self {
            id,
            video_file_id,
            title,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
