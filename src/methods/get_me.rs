// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::User;
use crate::Bot;

impl Bot {
    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
    /// <https://core.telegram.org/bots/api#getme>
    pub fn get_me(&self) -> GetMeBuilder {
        GetMeBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct GetMeBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> GetMeBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self { bot }
    }

    pub async fn send(self) -> Result<User> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<User>("getMe", Some(&form)).await
    }
}