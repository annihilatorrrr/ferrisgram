// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Voice;

impl Voice {
    /// This function creates an empty struct for the object Voice.
    pub fn new() -> Self {
        Self {
            file_id: "".to_string(),
            file_unique_id: "".to_string(),
            duration: 0,
            mime_type: None,
            file_size: None,
        }
    }
}
impl Default for Voice {
    fn default() -> Self {
        Self::new()
    }
}