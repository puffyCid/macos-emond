//! Parse macOS Emond data
//!
//! Provides a simple library to parse macOS Emond persistence data.

use std::fs::read_dir;

use log::{error, warn};
use plist::{Dictionary, Value};
use serde::Serialize;

use crate::{
    actions::{self, command, send_email, send_notification},
    error::EmondError,
    util::{get_boolean_value, get_dictionary_values, get_string_value},
};

#[derive(Debug, Serialize)]
pub struct EmondData {
    pub name: String,
    pub enabled: bool,
    pub event_types: Vec<String>,
    pub start_time: String,
    pub allow_partial_criterion_match: bool,
    pub command_actions: Vec<command::Command>,
    pub log_actions: Vec<actions::log::Log>,
    pub send_email_actions: Vec<send_email::SendEmail>,
    pub send_sms_action: Vec<send_email::SendEmail>,
    pub send_notification: Vec<send_notification::SendNotification>,
    pub criterion: Vec<Dictionary>,
    pub variables: Vec<Dictionary>,
    pub emond_clients_enabled: bool,
}

#[derive(Debug)]
struct Actions {
    command_actions: Vec<command::Command>,
    log_actions: Vec<actions::log::Log>,
    send_email_actions: Vec<send_email::SendEmail>,
    send_sms_action: Vec<send_email::SendEmail>,
    send_notification: Vec<send_notification::SendNotification>,
}

impl EmondData {
    /// Parse all Emond rules files at provided path
    pub fn parse_emond_rules(path: &str) -> Result<Vec<EmondData>, EmondError> {
        let dir_results = read_dir(path);

        let read_dir = match dir_results {
            Ok(dir) => dir,
            Err(err) => {
                error!("Failed to read Emond rules directory: {:?}", err);
                return Err(EmondError::Path);
            }
        };

        let mut emond_results: Vec<EmondData> = Vec::new();
        for dir in read_dir {
            let entry = match dir {
                Ok(results) => results,
                Err(err) => {
                    error!(
                        "Could not get file entry in Emond rules directory {}. Issue: {:?}",
                        path, err
                    );
                    return Err(EmondError::Path);
                }
            };

            let emond_rule_path = entry.path().display().to_string();

            let emond_data_results = EmondData::parse_emond_data(&emond_rule_path);
            let mut emond_data = match emond_data_results {
                Ok(results) => results,
                Err(err) => {
                    error!(
                        "Failed to parse Emond file: {}. Error: {}",
                        emond_rule_path,
                        err.to_string()
                    );
                    continue;
                }
            };
            emond_results.append(&mut emond_data);
        }
        Ok(emond_results)
    }

    /// Parse a single Emond rule file
    pub fn parse_emond_data(path: &str) -> Result<Vec<EmondData>, EmondError> {
        let mut emond_data_vec: Vec<EmondData> = Vec::new();
        let emond_plist_result = plist::from_file(path);
        let emond_plist = match emond_plist_result {
            Ok(result) => result,
            Err(err) => {
                error!("Failed to parse Emond PLIST Rule: {:?}", err);
                return Err(EmondError::Plist);
            }
        };

        match emond_plist {
            // Emond file may contain multiple rules as in an array
            Value::Array(plist_array) => {
                let mut emond_data = EmondData {
                    name: String::new(),
                    enabled: false,
                    event_types: Vec::new(),
                    command_actions: Vec::new(),
                    log_actions: Vec::new(),
                    send_email_actions: Vec::new(),
                    send_sms_action: Vec::new(),
                    send_notification: Vec::new(),
                    criterion: Vec::new(),
                    variables: Vec::new(),
                    allow_partial_criterion_match: false,
                    start_time: String::new(),
                    emond_clients_enabled: false,
                };

                for plist_values in plist_array {
                    match plist_values {
                        Value::Dictionary(plist_dictionary) => {
                            // Get the data in the Rule
                            for (key, value) in plist_dictionary {
                                if key == "eventTypes" {
                                    emond_data.event_types =
                                        EmondData::parse_event_types(&value).unwrap();
                                } else if key == "enabled" {
                                    emond_data.enabled = get_boolean_value(&value);
                                } else if key == "allowPartialCriterionMatch" {
                                    emond_data.allow_partial_criterion_match =
                                        get_boolean_value(&value);
                                } else if key == "criterion" {
                                    emond_data.criterion = get_dictionary_values(value);
                                } else if key == "startTime" {
                                    emond_data.start_time = get_string_value(&value);
                                } else if key == "variables" {
                                    emond_data.variables = get_dictionary_values(value);
                                } else if key == "name" {
                                    emond_data.name = get_string_value(&value);
                                } else if key == "actions" {
                                    let actions_results = EmondData::parse_actions(&value);
                                    let actions = match actions_results {
                                        Ok(results) => results,
                                        Err(err) => {
                                            warn!(
                                                "Failed to parse Emond Action data: {}",
                                                err.to_string()
                                            );
                                            continue;
                                        }
                                    };

                                    emond_data.log_actions = actions.log_actions;
                                    emond_data.command_actions = actions.command_actions;
                                    emond_data.send_email_actions = actions.send_email_actions;
                                    emond_data.send_notification = actions.send_notification;
                                } else {
                                    warn!(
                                        "Unknown key value ({}) in Emond Rule. Value: {:?}",
                                        key, value
                                    );
                                }
                            }
                        }
                        _ => continue,
                    }
                }

                let clients_results = EmondData::check_clients();
                emond_data.emond_clients_enabled = match clients_results {
                    Ok(results) => results,
                    Err(err) => {
                        warn!("Failed to find Emond client(s): {:?}", err.to_string());
                        false
                    }
                };
                emond_data_vec.push(emond_data);
            }
            _ => {
                warn!("Failed to get Emond Rule Array value");
                return Err(EmondError::Rule);
            }
        }
        Ok(emond_data_vec)
    }

    // Get the
    fn parse_event_types(value: &Value) -> Result<Vec<String>, EmondError> {
        let event_types_results = value.as_array();
        let mut event_types_vec: Vec<String> = Vec::new();
        match event_types_results {
            Some(events) => {
                for event in events {
                    let event_type_string = get_string_value(event);
                    event_types_vec.push(event_type_string);
                }
                Ok(event_types_vec)
            }
            None => {
                error!("Failed to parse Emond Event Types");
                Err(EmondError::EventType)
            }
        }
    }

    // Parse all Emond Actions
    fn parse_actions(value: &Value) -> Result<Actions, EmondError> {
        let mut emond_actions = Actions {
            command_actions: Vec::new(),
            log_actions: Vec::new(),
            send_email_actions: Vec::new(),
            send_sms_action: Vec::new(),
            send_notification: Vec::new(),
        };

        let value_array_results = value.as_array();
        let value_array = match value_array_results {
            Some(results) => results,
            None => {
                error!("Failed to parse Action array");
                return Err(EmondError::ActionArray);
            }
        };

        for value_data in value_array {
            let action_dictionary_results = value_data.as_dictionary();
            let action_dictionary = match action_dictionary_results {
                Some(results) => results,
                None => {
                    error!("Failed to parse Action Dictionary");
                    return Err(EmondError::ActionDictionary);
                }
            };

            for (key, action_value) in action_dictionary {
                if key != "type" {
                    continue;
                }
                let action_type = get_string_value(action_value);

                match action_type.as_str() {
                    "Log" => {
                        let log_data = actions::log::Log::parse_action_log(action_dictionary);
                        emond_actions.log_actions.push(log_data);
                    }
                    "RunCommand" => {
                        let command_data =
                            command::Command::parse_action_run_command(action_dictionary);
                        emond_actions.command_actions.push(command_data);
                    }
                    "SendEmail" => {
                        let email_data =
                            send_email::SendEmail::parse_action_send_email(action_dictionary);
                        emond_actions.send_sms_action.push(email_data);
                    }
                    "SendSMS" => {
                        // SendSMS apears to use same keys and values as Email?
                        // https://magnusviri.com/what-is-emond.html
                        let email_data =
                            send_email::SendEmail::parse_action_send_email(action_dictionary);
                        emond_actions.send_email_actions.push(email_data);
                    }
                    "SendNotification" => {
                        let notification_data =
                            send_notification::SendNotification::parse_action_send_notification(
                                action_dictionary,
                            );
                        emond_actions.send_notification.push(notification_data);
                    }
                    _ => warn!("Unknown Action Type: {}", action_type),
                }
            }
        }
        Ok(emond_actions)
    }

    // Check for any files in EmondClients directory
    // Emond will only run if a file is present
    fn check_clients() -> Result<bool, EmondError> {
        let client_path = "/private/var/db/emondClients";
        let dir_results = read_dir(client_path);

        let read_dir = match dir_results {
            Ok(dir) => dir,
            Err(err) => {
                error!("Failed to read Emond clients directory: {:?}", err);
                return Err(EmondError::Path);
            }
        };

        for dir in read_dir {
            let entry = match dir {
                Ok(results) => results,
                Err(err) => {
                    error!(
                        "Could not get file entry in Emond client directory {}. Issue: {:?}",
                        client_path, err
                    );
                    return Err(EmondError::Path);
                }
            };

            if entry.path().is_file() {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use plist::{Dictionary, Value};

    use crate::emond::EmondData;

    #[test]
    fn test_system_parse_emond_rules() {
        let default_path = "/etc/emond.d/rules";
        let results = EmondData::parse_emond_rules(default_path).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_parse_emond_rules() {
        let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_location.push("tests/test_data/");

        let results = EmondData::parse_emond_rules(&test_location.display().to_string()).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].enabled, true);
        assert_eq!(results[0].name, "poisonapple rule");
        assert_eq!(results[0].event_types, ["startup"]);
        assert_eq!(results[0].allow_partial_criterion_match, false);
        assert_eq!(results[0].criterion.is_empty(), true);
        assert_eq!(results[0].log_actions.is_empty(), true);
        assert_eq!(results[0].send_notification.is_empty(), true);
        assert_eq!(results[0].send_email_actions.is_empty(), true);
        assert_eq!(results[0].variables.is_empty(), true);

        assert_eq!(results[0].command_actions.len(), 1);
        assert_eq!(results[0].command_actions[0].command, "/Users/sur/Library/Python/3.8/lib/python/site-packages/poisonapple/auxiliary/poisonapple.sh");
        assert_eq!(results[0].command_actions[0].group, String::new());
        assert_eq!(results[0].command_actions[0].user, "root");
        assert_eq!(results[0].command_actions[0].arguements, ["Emond"]);

        assert_eq!(results[1].enabled, false);
        assert_eq!(results[1].name, "sample rule");
        assert_eq!(results[1].event_types, ["startup"]);
        assert_eq!(results[1].allow_partial_criterion_match, false);
        assert_eq!(results[1].criterion.len(), 1);

        let mut test_dictionary = Dictionary::new();
        test_dictionary.insert(
            String::from("operator"),
            Value::String(String::from("True")),
        );

        assert_eq!(results[1].criterion[0], test_dictionary);

        assert_eq!(results[1].send_notification.is_empty(), true);
        assert_eq!(results[1].send_email_actions.is_empty(), true);
        assert_eq!(results[1].variables.is_empty(), true);
        assert_eq!(results[1].command_actions.is_empty(), true);

        assert_eq!(results[1].log_actions.len(), 1);

        assert_eq!(
            results[1].log_actions[0].message,
            "Event Monitor started at ${builtin:now}"
        );
        assert_eq!(results[1].log_actions[0].facility, String::new());
        assert_eq!(results[1].log_actions[0].log_level, "Notice");
        assert_eq!(results[1].log_actions[0].log_type, "syslog");
        assert_eq!(results[1].log_actions[0].parameters, Dictionary::new());
    }

    #[test]
    fn test_parse_emond_data() {
        let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_location.push("tests/test_data/test123.plist");

        let results = EmondData::parse_emond_data(&test_location.display().to_string()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].enabled, true);
        assert_eq!(results[0].name, "poisonapple rule");
        assert_eq!(results[0].event_types, ["startup"]);
        assert_eq!(results[0].allow_partial_criterion_match, false);
        assert_eq!(results[0].criterion.is_empty(), true);
        assert_eq!(results[0].log_actions.is_empty(), true);
        assert_eq!(results[0].send_notification.is_empty(), true);
        assert_eq!(results[0].send_email_actions.is_empty(), true);
        assert_eq!(results[0].variables.is_empty(), true);

        assert_eq!(results[0].command_actions.len(), 1);
        assert_eq!(results[0].command_actions[0].command, "/Users/sur/Library/Python/3.8/lib/python/site-packages/poisonapple/auxiliary/poisonapple.sh");
        assert_eq!(results[0].command_actions[0].group, String::new());
        assert_eq!(results[0].command_actions[0].user, "root");
        assert_eq!(results[0].command_actions[0].arguements, ["Emond"]);
    }

    #[test]
    fn test_parse_event_types() {
        let test: Value = Value::Array(vec![
            Value::String(String::from("startup")),
            Value::String(String::from("auth:login")),
        ]);

        let results = EmondData::parse_event_types(&test).unwrap();
        assert_eq!(results[0], "startup");
        assert_eq!(results[1], "auth:login");
    }

    #[test]
    #[ignore = "Check live system if Emond is enabled"]
    fn test_check_clients() {
        let results = EmondData::check_clients().unwrap();
        assert_eq!(results, false);
    }

    #[test]
    fn test_parse_actions() {
        let mut test_dictionary = Dictionary::new();
        test_dictionary.insert(String::from("message"), Value::String(String::from("test")));
        test_dictionary.insert(
            String::from("command"),
            Value::String(String::from("nc -l")),
        );
        test_dictionary.insert(String::from("user"), Value::String(String::from("root")));
        test_dictionary.insert(String::from("arguments"), Value::Array(Vec::new()));
        test_dictionary.insert(String::from("group"), Value::String(String::from("wheel")));
        test_dictionary.insert(
            String::from("type"),
            Value::String(String::from("RunCommand")),
        );

        let test_value: Value = Value::Array(vec![plist::Value::Dictionary(test_dictionary)]);

        let results = EmondData::parse_actions(&test_value).unwrap();
        assert_eq!(results.command_actions[0].user, "root");
        assert_eq!(results.command_actions[0].group, "wheel");
        assert_eq!(results.command_actions[0].command, "nc -l");
        assert_eq!(results.command_actions[0].arguements.len(), 0);
    }
}
