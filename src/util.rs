use log::warn;
use plist::{Dictionary, Value};

// Get the string value from the dictionary
pub fn get_string_value(dict_data: &Value) -> String {
    let results = dict_data.as_string();
    match results {
        Some(data) => data.to_string(),
        None => {
            warn!("No string value in PLIST file");
            String::new()
        }
    }
}

// Get the bool value from the dictionary
pub fn get_boolean_value(dict_data: &Value) -> bool {
    let results = dict_data.as_boolean();
    match results {
        Some(data) => data,
        None => {
            warn!("No bool value in PLIST file");
            false
        }
    }
}

// Get the Dictionary value from the dictionary
pub fn get_dictionary_value(dict_data: Value) -> Dictionary {
    let results = dict_data.into_dictionary();
    match results {
        Some(data) => data,
        None => {
            warn!("No dictionary value in PLIST file");
            Dictionary::new() // or return error?
        }
    }
}

// Get the Vec of dictionaries value from the dictionary
pub fn get_dictionary_values(dict_data: Value) -> Vec<Dictionary> {
    let mut dictionary_vec: Vec<Dictionary> = Vec::new();
    let results = dict_data.into_array();
    match results {
        Some(data) => {
            for value in data {
                let dictionary_value = get_dictionary_value(value);
                dictionary_vec.push(dictionary_value);
            }
            dictionary_vec
        }
        None => {
            warn!("No dictionary array in PLIST file");
            dictionary_vec
        }
    }
}

#[cfg(test)]
mod tests {
    use plist::{Dictionary, Value};

    use crate::util::{
        get_boolean_value, get_dictionary_value, get_dictionary_values, get_string_value,
    };

    #[test]
    fn test_get_string_value() {
        let test: Value = Value::String(String::from("test"));
        let results = get_string_value(&test);

        assert_eq!(results, "test");
    }

    #[test]
    fn test_get_bool_value() {
        let test: Value = Value::Boolean(false);
        let results = get_boolean_value(&test);

        assert_eq!(results, false);
    }

    #[test]
    fn test_get_dictionary_value() {
        let test: Value = Value::Dictionary(Dictionary::new());
        let results = get_dictionary_value(test);

        assert_eq!(results, Dictionary::new());
    }

    #[test]
    fn test_get_dictionary_values() {
        let test: Value = Value::Dictionary(Dictionary::new());
        let test_array: Value = Value::Array(vec![test]);
        let results = get_dictionary_values(test_array);

        assert_eq!(results.len(), 1);
    }
}
