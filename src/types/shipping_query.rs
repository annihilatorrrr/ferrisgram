// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{ShippingAddress, User};
use serde::{Deserialize, Serialize};

/// This object contains information about an incoming shipping query.
/// <https://core.telegram.org/bots/api#shippingquery>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}