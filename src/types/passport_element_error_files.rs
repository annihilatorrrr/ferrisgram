// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
/// <https://core.telegram.org/bots/api#passportelementerrorfiles>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportElementErrorFiles {
    /// Error source, must be files
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    pub r#type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}