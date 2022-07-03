use log::warn;
use plist::Dictionary;
use serde::Serialize;

use crate::util::get_string_value;

#[derive(Debug, Serialize)]
pub struct SendEmail {
    pub message: String,
    pub subject: String,
    pub localization_bundle_path: String,
    pub relay_host: String,
    pub admin_email: String,
    pub recipient_addresses: Vec<String>,
}

impl SendEmail {
    pub fn parse_action_send_email(action_dictionary: &Dictionary) -> SendEmail {
        let mut email_data = SendEmail {
            message: String::new(),
            subject: String::new(),
            localization_bundle_path: String::new(),
            relay_host: String::new(),
            admin_email: String::new(),
            recipient_addresses: Vec::new(),
        };

        for (key, action_value) in action_dictionary {
            if key == "message" {
                email_data.message = get_string_value(action_value);
            } else if key == "subject" {
                email_data.subject = get_string_value(action_value);
            } else if key == "localization_bundle_path" {
                email_data.localization_bundle_path = get_string_value(action_value);
            } else if key == "relay_host" {
                email_data.relay_host = get_string_value(action_value);
            } else if key == "admin_email" {
                email_data.admin_email = get_string_value(action_value);
            } else if key == "recipient_addresses" {
                let arg_results = action_value.as_array();
                let arg_array = match arg_results {
                    Some(results) => results,
                    None => {
                        warn!(
                            "Failed to parse Send Email Action array: {:?}",
                            action_value
                        );
                        continue;
                    }
                };

                for args in arg_array {
                    email_data.recipient_addresses.push(get_string_value(args));
                }
            } else if key == "type" {
                // Skip type values. We already know the action type
                continue;
            } else {
                warn!("Unknown Log Action key: {}. Value: {:?}", key, action_value);
            }
        }
        email_data
    }
}
