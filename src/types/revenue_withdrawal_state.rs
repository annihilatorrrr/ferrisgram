// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{
    RevenueWithdrawalStateFailed, RevenueWithdrawalStatePending, RevenueWithdrawalStateSucceeded,
};
use serde::{Deserialize, Serialize};

/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
/// - RevenueWithdrawalStatePending
/// - RevenueWithdrawalStateSucceeded
/// - RevenueWithdrawalStateFailed
/// <https://core.telegram.org/bots/api#revenuewithdrawalstate>
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RevenueWithdrawalState {
    #[serde(rename = "pending")]
    RevenueWithdrawalStatePending(RevenueWithdrawalStatePending),
    #[serde(rename = "succeeded")]
    RevenueWithdrawalStateSucceeded(RevenueWithdrawalStateSucceeded),
    #[serde(rename = "failed")]
    RevenueWithdrawalStateFailed(RevenueWithdrawalStateFailed),
}
