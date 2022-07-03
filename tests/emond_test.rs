use std::path::PathBuf;

use plist::{Dictionary, Value};

#[test]
fn test_parse_emond_rules() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/");

    let results =
        macos_emond::parser::parse_emond_rules(&test_location.display().to_string()).unwrap();
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
#[should_panic(expected = "Plist")]
fn test_bad_rule_plist() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/bad_data/test123.plist");

    let results =
        macos_emond::parser::parse_emond_file(&test_location.display().to_string()).unwrap();
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

    assert_eq!(results[0].command_actions.is_empty(), true);
}

#[test]
#[should_panic(expected = "Plist")]
fn test_bad_plist() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/bad_data/bad_plist.plist");

    let _ = macos_emond::parser::parse_emond_file(&test_location.display().to_string()).unwrap();
}

#[test]
#[should_panic(expected = "Path")]
fn test_bad_directory() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/abc");

    let _ = macos_emond::parser::parse_emond_rules(&test_location.display().to_string()).unwrap();
}
