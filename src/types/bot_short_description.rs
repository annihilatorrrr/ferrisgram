// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// This object represents the bot's short description.
/// <https://core.telegram.org/bots/api#botshortdescription>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: String,
}