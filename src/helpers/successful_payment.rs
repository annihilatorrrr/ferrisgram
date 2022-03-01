// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::SuccessfulPayment;

impl SuccessfulPayment {
    /// This function creates an empty struct for the object SuccessfulPayment.
    pub fn new() -> Self {
        Self {
            currency: "".to_string(),
            total_amount: 0,
            invoice_payload: "".to_string(),
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id: "".to_string(),
            provider_payment_charge_id: "".to_string(),
        }
    }
}
impl Default for SuccessfulPayment {
    fn default() -> Self {
        Self::new()
    }
}