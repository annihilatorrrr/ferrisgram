// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::StarTransactions;
use crate::Bot;

impl Bot {
    /// Returns the bot's Telegram Star transactions in chronological order. On success, returns a StarTransactions object.
    /// <https://core.telegram.org/bots/api#getstartransactions>
    pub fn get_star_transactions(&self) -> GetStarTransactionsBuilder {
        GetStarTransactionsBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct GetStarTransactionsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Number of transactions to skip in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl<'a> GetStarTransactionsBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            offset: None,
            limit: None,
        }
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn send(self) -> Result<StarTransactions> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getStarTransactions", Some(&form)).await
    }
}
