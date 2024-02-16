// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{Chat, User};
use serde::{Deserialize, Serialize};

/// This object represents a message about the completion of a giveaway with public winners.
/// <https://core.telegram.org/bots/api#giveawaywinners>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway
    pub chat: Chat,
    /// Identifier of the message with the giveaway in the chat
    pub giveaway_message_id: i64,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: i64,
    /// Total number of winners in the giveaway
    pub winner_count: i64,
    /// List of up to 100 winners of the giveaway
    pub winners: Vec<User>,
    /// Optional. The number of other chats the user had to join in order to be eligible for the giveaway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i64>,
    /// Optional. The number of months the Telegram Premium subscription won from the giveaway will be active for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
    /// Optional. Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    /// Optional. True, if only users who had joined the chats after the giveaway started were eligible to win
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// Optional. True, if the giveaway was canceled because the payment for it was refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// Optional. Description of additional giveaway prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}
