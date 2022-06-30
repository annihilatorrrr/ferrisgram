// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::MenuButton;

impl Bot {
    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
    /// <https://core.telegram.org/bots/api#setchatmenubutton>
    pub fn set_chat_menu_button(&self) -> SetChatMenuButtonBuilder {
        SetChatMenuButtonBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct SetChatMenuButtonBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// A JSON-serialized object for the bot's new menu button. Defaults to MenuButtonDefault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}


impl <'a> SetChatMenuButtonBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self{
            bot,
            chat_id: None,
            menu_button: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
                
    pub fn menu_button(mut self, menu_button: MenuButton) -> Self {
        self.menu_button = Some(menu_button);
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("setChatMenuButton", Some(&form)).await
    }

}