use log::warn;
use plist::Dictionary;
use serde::Serialize;

use crate::util::get_string_value;

#[derive(Debug, Serialize)]
pub struct Command {
    pub command: String,
    pub user: String,
    pub group: String,
    pub arguements: Vec<String>,
}

impl Command {
    // Parse the Run Command Action Emond Rule
    pub fn parse_action_run_command(action_dictionary: &Dictionary) -> Command {
        let mut command_data = Command {
            command: String::new(),
            user: String::new(),
            group: String::new(),
            arguements: Vec::new(),
        };
        for (key, action_value) in action_dictionary {
            if key == "command" {
                command_data.command = get_string_value(action_value);
            } else if key == "user" {
                command_data.user = get_string_value(action_value);
            } else if key == "group" {
                command_data.group = get_string_value(action_value);
            } else if key == "arguments" {
                let arg_results = action_value.as_array();
                let arg_array = match arg_results {
                    Some(results) => results,
                    None => {
                        warn!(
                            "Failed to parse Run Command Action array: {:?}",
                            action_value
                        );
                        continue;
                    }
                };

                for args in arg_array {
                    command_data.arguements.push(get_string_value(args));
                }
            } else if key == "type" {
                // Skip type values. We already know the action type
                continue;
            }
        }
        command_data
    }
}

#[cfg(test)]
mod tests {
    use plist::{Dictionary, Value};

    use crate::actions::command;

    #[test]
    fn test_parse_action_run_command() {
        let mut test_dictionary = Dictionary::new();
        test_dictionary.insert(String::from("message"), Value::String(String::from("test")));
        test_dictionary.insert(
            String::from("command"),
            Value::String(String::from("nc -l")),
        );
        test_dictionary.insert(String::from("user"), Value::String(String::from("root")));
        test_dictionary.insert(String::from("arguments"), Value::Array(Vec::new()));
        test_dictionary.insert(String::from("group"), Value::String(String::from("wheel")));

        let results = command::Command::parse_action_run_command(&test_dictionary);
        assert_eq!(results.user, "root");
        assert_eq!(results.group, "wheel");
        assert_eq!(results.command, "nc -l");
        assert_eq!(results.arguements.len(), 0);
    }
}
