// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::InputInvoiceMessageContent;
use crate::types::LabeledPrice;

impl InputInvoiceMessageContent {
    /// This function creates an empty struct for the object InputInvoiceMessageContent.
    pub fn new(
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            title,
            description,
            payload,
            provider_token: None,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }
}
