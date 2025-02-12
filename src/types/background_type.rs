// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{
    BackgroundTypeChatTheme, BackgroundTypeFill, BackgroundTypePattern, BackgroundTypeWallpaper,
};
use serde::{Deserialize, Serialize};

/// This object describes the type of a background. Currently, it can be one of
/// - BackgroundTypeFill
/// - BackgroundTypeWallpaper
/// - BackgroundTypePattern
/// - BackgroundTypeChatTheme
/// <https://core.telegram.org/bots/api#backgroundtype>
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BackgroundType {
    #[serde(rename = "fill")]
    BackgroundTypeFill(BackgroundTypeFill),
    #[serde(rename = "wallpaper")]
    BackgroundTypeWallpaper(BackgroundTypeWallpaper),
    #[serde(rename = "pattern")]
    BackgroundTypePattern(BackgroundTypePattern),
    #[serde(rename = "chat_theme")]
    BackgroundTypeChatTheme(BackgroundTypeChatTheme),
}
