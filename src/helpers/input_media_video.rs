// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::InputMediaVideo;

impl InputMediaVideo {
    /// This function creates an empty struct for the object InputMediaVideo.
    pub fn new(media: String) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            has_spoiler: None,
        }
    }
}
