use std::collections::HashMap;
use crate::json_reader::JsonValue::{JsonBoolean, JsonFloat, JsonInt, JsonString};

#[derive(Debug, PartialEq)]
enum JsonValue {
    JsonInt(i32),
    JsonFloat(f64),
    JsonString(String),
    JsonBoolean(bool)
}
pub fn read(json: &str) -> Option<HashMap<String, JsonValue>> {
    let mut words: Vec<&str>  = json.split_whitespace().collect();
    println!("${:?}", words);
    let size = words.len();
    let mut index = 0;
    let mut result: HashMap<String, JsonValue> = HashMap::new();

    // run
    while index < size {
        match words[index] {
                "{}" => break,
                "{" | "," => {
                    // get key (n + 1), skip separator(n + 2), get value(n + 3), if none of these exist, panic
                    index = index + 1;
                    let key: &str = words[index];
                    index = index + 2;
                    let mut value: &str = words[index];
                    // if value ends with , or }, remove it and add it to words at index + 1
                    [",", "}"].iter().for_each(|c| {
                        if value.ends_with(c) {
                            value = &value[..value.len()-1];
                            words.insert(index + 1, c);
                            println!("checking {c} {:?}", words);
                        }
                    });

                    if (is_int(value)) {
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonInt(value.parse::<i32>().unwrap())
                        );
                    } else if (is_float(value)) {
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonFloat(value.parse::<f64>().unwrap())
                        );
                    } else if (is_bool(value)) {
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonBoolean(value.parse::<bool>().unwrap())
                        );
                    } else {
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonString(value.replace("\"", "").to_string())
                        );
                    }
                },
                _ => break
            }
        index = index + 1;
    }

    if result.is_empty(){
        None
    }
    else {
        Some(result)
    }
}

fn is_int(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn is_float(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn is_bool(s: &str) -> bool {
    s.parse::<bool>().is_ok()
}



mod tests {
    use std::collections::HashMap;
    use crate::json_reader::JsonValue::{JsonInt, JsonString};
    use super::*;

    #[test]
    fn read_empty_json() {
        let json = "{}";
        let result = read(&json);
        assert!(result.is_none());
    }

    #[test]
    fn read_basic_to_map () {
        let json = r#"{ "key" : "value" }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonString("value".to_string()));
        let result = read(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_multiple_to_map () {
        let json = r#"{ "key1" : "value1" , "key2" : "value2" }"#; // need the case where its value,
        let mut expected = HashMap::new();
        expected.insert("key1".to_string() , JsonString("value1".to_string()));
        expected.insert("key2".to_string() , JsonString("value2".to_string()));
        let result = read(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_multiple_to_map_no_space () {
        let json = r#"{ "key1" : "value1", "key2" : "value2"}"#; // need the case where its value,
        let mut expected = HashMap::new();
        expected.insert("key1".to_string() , JsonString("value1".to_string()));
        expected.insert("key2".to_string() , JsonString("value2".to_string()));
        let result = read(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_to_map_with_number () {
        let json = r#"{ "key" : 5 }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonInt(5));
        let result = read(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_to_map_with_float () {
        let json = r#"{ "key" : 5.0 }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonFloat(5.0));
        let result = read(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_to_map_with_bool () {
        let json = r#"{ "key1" : true, "key2" : false }"#;
        let mut expected = HashMap::new();
        expected.insert("key1".to_string() , JsonBoolean(true));
        expected.insert("key2".to_string() , JsonBoolean(false));
        let result = read(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_different_types () {
        let json = r#"{ "key1" : true, "key2" : false, "key3" : 5.0 , "key4" : 5, "key5" : "value5" }"#;
        let mut expected = HashMap::new();
        expected.insert("key1".to_string() , JsonBoolean(true));
        expected.insert("key2".to_string() , JsonBoolean(false));
        expected.insert("key3".to_string() , JsonFloat(5.0));
        expected.insert("key4".to_string() , JsonInt(5));
        expected.insert("key5".to_string() , JsonString("value5".to_string()));
        let result = read(&json);
        assert_eq!(expected, result.unwrap());
    }
}