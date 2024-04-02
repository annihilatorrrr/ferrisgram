// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::User;

impl User {
    /// This function creates an empty struct for the object User.
    pub fn new(id: i64, is_bot: bool, first_name: String) -> Self {
        Self {
            id,
            is_bot,
            first_name,
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
            can_connect_to_business: None,
        }
    }
}
