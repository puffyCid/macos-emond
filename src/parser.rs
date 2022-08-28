use log::error;
use plist::{Dictionary, Value};

use crate::{emond::EmondData, error::EmondError, size::get_file_size, util};

/// Parse the Emond Config PLIST to get any additional Emond Rules directories besides the default path
pub fn get_emond_rules_paths() -> Result<Vec<String>, EmondError> {
    let emond_plist_path: String = String::from("/etc/emond.d/emond.plist");
    if !get_file_size(&emond_plist_path) {
        return Ok(Vec::new());
    }
    let emond_plist_result: Result<Dictionary, plist::Error> = plist::from_file(emond_plist_path);

    let emond_plist = match emond_plist_result {
        Ok(results) => results,
        Err(err) => {
            error!("Failed to parse Emond Config PLIST file: {:?}", err);
            return Err(EmondError::Plist);
        }
    };

    let mut emond_rules_paths: Vec<String> = Vec::new();
    let default_path = String::from("/etc/emond.d/rules");
    emond_rules_paths.push(default_path);

    for (key, value) in emond_plist {
        if key != "config" {
            continue;
        }
        // Parse the config dictionary and get all the additional paths at additionalRulesPaths
        match value {
            Value::Dictionary(value_dictionary) => {
                for (subkey, subvalue) in value_dictionary {
                    if subkey == "additionalRulesPaths" {
                        // Additional paths are stored as an array. Loop and get all the paths (if any)
                        match subvalue {
                            Value::Array(value_array) => {
                                for additional_path in value_array {
                                    let path_string = util::get_string_value(&additional_path);
                                    emond_rules_paths.push(path_string.to_string());
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    Ok(emond_rules_paths)
}

/// Parse all the Emond Rules at provided path
pub fn parse_emond_rules(path: &str) -> Result<Vec<EmondData>, EmondError> {
    EmondData::parse_emond_rules(path)
}

/// Parse the Emond Rules file at provided path
pub fn parse_emond_file(path: &str) -> Result<Vec<EmondData>, EmondError> {
    EmondData::parse_emond_data(path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use plist::{Dictionary, Value};

    use crate::parser::parse_emond_file;

    use super::{get_emond_rules_paths, parse_emond_rules};

    #[test]
    fn test_get_emond_rules_paths() {
        let results = get_emond_rules_paths().unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "/etc/emond.d/rules");
    }

    #[test]
    fn test_parse_emond_rules() {
        let default_path = "/etc/emond.d/rules";
        let results = parse_emond_rules(default_path).unwrap();
        println!("{:?}", results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].enabled, false);
        assert_eq!(results[0].name, "sample rule");
        assert_eq!(results[0].event_types, ["startup"]);
        assert_eq!(results[0].allow_partial_criterion_match, false);
        assert_eq!(results[0].criterion.len(), 1);

        let mut test_dictionary = Dictionary::new();
        test_dictionary.insert(
            String::from("operator"),
            Value::String(String::from("True")),
        );

        assert_eq!(results[0].criterion[0], test_dictionary);

        assert_eq!(results[0].send_notification.is_empty(), true);
        assert_eq!(results[0].send_email_actions.is_empty(), true);
        assert_eq!(results[0].variables.is_empty(), true);
        assert_eq!(results[0].command_actions.is_empty(), true);

        assert_eq!(results[0].log_actions.len(), 1);

        assert_eq!(
            results[0].log_actions[0].message,
            "Event Monitor started at ${builtin:now}"
        );
        assert_eq!(results[0].log_actions[0].facility, String::new());
        assert_eq!(results[0].log_actions[0].log_level, "Notice");
        assert_eq!(results[0].log_actions[0].log_type, "syslog");
        assert_eq!(results[0].log_actions[0].parameters, Dictionary::new());
    }

    #[test]
    fn test_parse_emond_data() {
        let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_location.push("tests/test_data/test123.plist");

        let results = parse_emond_file(&test_location.display().to_string()).unwrap();
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
}
