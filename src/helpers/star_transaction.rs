// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::StarTransaction;

impl StarTransaction {
    /// This function creates an empty struct for the object StarTransaction.
    pub fn new(id: String, amount: i64, date: i64) -> Self {
        Self {
            id,
            amount,
            date,
            source: None,
            receiver: None,
        }
    }
}
