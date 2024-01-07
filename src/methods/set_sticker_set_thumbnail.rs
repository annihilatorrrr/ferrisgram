// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
// use crate::types::InputFile;
use crate::Bot;

impl Bot {
    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns True on success.
    /// <https://core.telegram.org/bots/api#setstickersetthumbnail>
    pub fn set_sticker_set_thumbnail(
        &self,
        name: String,
        user_id: i64,
    ) -> SetStickerSetThumbnailBuilder {
        SetStickerSetThumbnailBuilder::new(self, name, user_id)
    }
}

#[derive(Serialize)]
pub struct SetStickerSetThumbnailBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A .WEBP or .PNG image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a .TGS animation with a thumbnail up to 32 kilobytes in size (see https://core.telegram.org/stickers#animated-sticker-requirements for animated sticker technical requirements), or a WEBM video with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#video-sticker-requirements for video sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files. Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

impl<'a> SetStickerSetThumbnailBuilder<'a> {
    pub fn new(bot: &'a Bot, name: String, user_id: i64) -> Self {
        Self {
            bot,
            name,
            user_id,
            thumbnail: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn thumbnail(mut self, thumbnail: String) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get::<bool>("setStickerSetThumbnail", Some(&form))
            .await
    }
}
