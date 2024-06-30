// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::TransactionPartner;
use serde::{Deserialize, Serialize};

/// Describes a Telegram Star transaction.
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifer of the original transaction for refund transactions. Coincides with SuccessfulPayment.telegram_payment_charge_id for successful incoming payments from users.
    pub id: String,
    /// Number of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// Date the transaction was created in Unix time
    pub date: i64,
    /// Optional. Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,
    /// Optional. Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,
}