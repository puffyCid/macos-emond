use log::warn;
use plist::Dictionary;
use serde::Serialize;

use crate::util::{get_dictionary_value, get_string_value};

#[derive(Debug, Serialize)]
pub struct SendNotification {
    pub name: String,
    pub message: String,
    pub details: Dictionary,
}

impl SendNotification {
    pub fn parse_action_send_notification(action_dictionary: &Dictionary) -> SendNotification {
        let mut notification = SendNotification {
            message: String::new(),
            name: String::new(),
            details: Dictionary::new(),
        };

        for (key, action_value) in action_dictionary {
            if key == "message" {
                notification.message = get_string_value(action_value);
            } else if key == "name" {
                notification.name = get_string_value(action_value);
            } else if key == "details" {
                notification.details = get_dictionary_value(action_value.clone());
            } else if key == "type" {
                // Skip type values. We already know the action type
                continue;
            } else {
                warn!("Unknown Log Action key: {}. Value: {:?}", key, action_value);
            }
        }
        notification
    }
}
