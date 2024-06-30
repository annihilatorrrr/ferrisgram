// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::StarTransaction;
use serde::{Deserialize, Serialize};

/// Contains a list of Telegram Star transactions.
/// <https://core.telegram.org/bots/api#startransactions>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarTransactions {
    /// The list of transactions
    pub transactions: Vec<StarTransaction>,
}