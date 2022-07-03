use log::warn;
use plist::Dictionary;
use serde::Serialize;

use crate::util::{get_dictionary_value, get_string_value};

#[derive(Debug, Serialize)]
pub struct Log {
    pub message: String,
    pub facility: String,
    pub log_level: String,
    pub log_type: String,
    pub parameters: Dictionary,
}

impl Log {
    // Parse the Log Action Emond Rule
    pub fn parse_action_log(action_dictionary: &Dictionary) -> Log {
        let mut log_data = Log {
            message: String::new(),
            facility: String::new(),
            log_level: String::new(),
            log_type: String::new(),
            parameters: Dictionary::new(),
        };

        for (key, action_value) in action_dictionary {
            if key == "message" {
                log_data.message = get_string_value(action_value);
            } else if key == "logLevel" {
                log_data.log_level = get_string_value(action_value);
            } else if key == "logType" {
                log_data.log_type = get_string_value(action_value);
            } else if key == "parameters" {
                log_data.parameters = get_dictionary_value(action_value.clone());
            } else if key == "facility" {
                log_data.facility = get_string_value(action_value);
            } else if key == "type" {
                // Skip type values. We already know the action type
                continue;
            } else {
                warn!("Unknown Log Action key: {}. Value: {:?}", key, action_value);
            }
        }
        log_data
    }
}

#[cfg(test)]
mod tests {
    use plist::{Dictionary, Value};

    use crate::actions::log::Log;

    #[test]
    fn test_parse_action_log() {
        let mut test_dictionary = Dictionary::new();
        test_dictionary.insert(String::from("message"), Value::String(String::from("test")));
        test_dictionary.insert(
            String::from("logLevel"),
            Value::String(String::from("level1")),
        );
        test_dictionary.insert(
            String::from("logType"),
            Value::String(String::from("type1")),
        );
        test_dictionary.insert(
            String::from("parameters"),
            Value::Dictionary(Dictionary::new()),
        );
        test_dictionary.insert(
            String::from("facility"),
            Value::String(String::from("testing")),
        );

        let results = Log::parse_action_log(&test_dictionary);
        assert_eq!(results.message, "test");
        assert_eq!(results.log_level, "level1");
        assert_eq!(results.log_type, "type1");
        assert_eq!(results.facility, "testing");
        assert_eq!(results.parameters, Dictionary::new());
    }
}
