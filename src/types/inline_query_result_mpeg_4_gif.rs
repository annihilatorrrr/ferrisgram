// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};
use serde::{Deserialize, Serialize};

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
/// <https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MPEG4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// Optional. MIME type of the thumbnail, must be one of "image/jpeg", "image/gif", or "video/mp4". Defaults to "image/jpeg"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
