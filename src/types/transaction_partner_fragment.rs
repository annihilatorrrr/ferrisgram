// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::RevenueWithdrawalState;
use serde::{Deserialize, Serialize};

/// Describes a withdrawal transaction with Fragment.
/// <https://core.telegram.org/bots/api#transactionpartnerfragment>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionPartnerFragment {
    /// Optional. State of the transaction if the transaction is outgoing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}
