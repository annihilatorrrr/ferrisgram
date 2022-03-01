// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::GameHighScore;
use crate::Bot;

impl Bot {
    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. On success, returns an Array of GameHighScore objects.
    /// <https://core.telegram.org/bots/api#getgamehighscores>
    pub fn get_game_high_scores(&self, user_id: i64) -> GetGameHighScoresBuilder {
        GetGameHighScoresBuilder::new(self, user_id)
    }
}

#[derive(Serialize)]
pub struct GetGameHighScoresBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Target user id
    pub user_id: i64,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl<'a> GetGameHighScoresBuilder<'a> {
    pub fn new(bot: &'a Bot, user_id: i64) -> Self {
        Self {
            bot,
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub async fn send(self) -> Result<Vec<GameHighScore>> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get::<Vec<GameHighScore>>("getGameHighScores", Some(&form))
            .await
    }
}