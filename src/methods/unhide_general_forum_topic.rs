// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    /// <https://core.telegram.org/bots/api#unhidegeneralforumtopic>
    pub fn unhide_general_forum_topic(&self, chat_id: i64) -> UnhideGeneralForumTopicBuilder {
        UnhideGeneralForumTopicBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct UnhideGeneralForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
}


impl <'a> UnhideGeneralForumTopicBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self{
            bot,
            chat_id,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("unhideGeneralForumTopic", Some(&form)).await
    }

}