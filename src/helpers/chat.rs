// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::Chat;

impl Chat {
    /// This function creates an empty struct for the object Chat.
    pub fn new(id: i64, r#type: String) -> Self {
        Self {
            id,
            r#type,
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            is_forum: None,
        }
    }
}
