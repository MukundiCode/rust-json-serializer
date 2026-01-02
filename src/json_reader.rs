use std::collections::HashMap;
use crate::json_reader::JsonValue::{JsonArray, JsonBoolean, JsonFloat, JsonInt, JsonObject, JsonString};

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    JsonInt(i32),
    JsonFloat(f64),
    JsonString(String),
    JsonBoolean(bool),
    JsonObject(HashMap<String, JsonValue>),
    JsonArray(Vec<JsonValue>)
}
pub fn read(json: &str) -> (Option<HashMap<String, JsonValue>>, usize) {
    let mut words: Vec<&str>  = split_preserving_quotes(json);
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

                    if is_int(value) {
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonInt(value.parse::<i32>().unwrap())
                        );
                    } else if is_float(value) {
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonFloat(value.parse::<f64>().unwrap())
                        );
                    } else if is_bool(value) {
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonBoolean(value.parse::<bool>().unwrap())
                        );
                    } else if is_json_object(value) {
                        // expect val to be '{...}'
                        let (obj, i) = read(&value[1..value.len()-1]);
                        match obj {
                            None => {}
                            Some(val) => {
                                result.insert(
                                    key.replace("\"", "").to_string(),
                                    JsonObject(val)
                                );
                                index = index + i;
                            }
                        }
                    } else if is_json_arr(value) {
                        // expect val to be '[...]',, need to remove '[ and ]' in this case
                        let items = split_on_commas_preserving_quotes(&value[2..value.len()-2]);
                        let mut json_arr: Vec<JsonValue> = Vec::new();
                        for item in items {
                            match item {
                                _val if is_int(item) => {
                                    json_arr.push(JsonInt(item.parse::<i32>().unwrap()))
                                }
                                _val if is_float(item) => {
                                    json_arr.push(JsonFloat(item.parse::<f64>().unwrap()));
                                }
                                _val if is_bool(item) => {
                                    json_arr.push(JsonBoolean(item.parse::<bool>().unwrap()));
                                }
                                _val if is_json_object(item) => {
                                    let (obj, _) = read(&item[1..item.len()-1]);
                                    match obj {
                                        None => {}
                                        Some(val) => {
                                            json_arr.push(JsonObject(val));
                                            // index = index + i;
                                        }
                                    }
                                }
                                _val if is_json_arr(item) => {

                                }
                                _ => {
                                    // assume string
                                    json_arr.push(JsonString(item.replace("\"", "").to_string()))
                                }
                            }
                        }

                        // finally, add this to the obj
                        result.insert(
                            key.replace("\"", "").to_string(),
                            JsonArray(json_arr)
                        );

                    }
                    else {
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
        (None, index)
    }
    else {
        (Some(result), index)
    }
}

fn read_helper(json: &str) -> Option<HashMap<String, JsonValue>> {
    let (ans, _) = read(&json);
    ans
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

fn is_json_object(s: &str) -> bool {
    s.starts_with("'{") && s.ends_with("}'")
}

fn is_json_arr(s: &str) -> bool {
    s.starts_with("'[") && s.ends_with("]'")
}

fn split_preserving_quotes(input: &str) -> Vec<&str> {
    let mut tokens = Vec::new();

    let mut in_quotes = false;
    let mut quote_char = '\0';

    let mut token_start: Option<usize> = None;

    for (i, c) in input.char_indices() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = c;
                token_start.get_or_insert(i);
            }
            c if in_quotes && c == quote_char => {
                in_quotes = false;
            }
            c if c.is_whitespace() && !in_quotes => {
                if let Some(start) = token_start {
                    tokens.push(&input[start..i]);
                    token_start = None;
                }
            }
            _ => {
                token_start.get_or_insert(i);
            }
        }
    }

    if let Some(start) = token_start {
        tokens.push(&input[start..]);
    }

    tokens
}

fn split_on_commas_preserving_quotes(input: &str) -> Vec<&str> {
    let mut tokens = Vec::new();

    let mut in_quotes = false;
    let mut quote_char = '\0';
    let mut token_start: Option<usize> = None;

    for (i, c) in input.char_indices() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = c;
                token_start.get_or_insert(i);
            }
            c if in_quotes && c == quote_char => {
                in_quotes = false;
            }
            ',' if !in_quotes => {
                if let Some(start) = token_start {
                    tokens.push(input[start..i].trim());
                    token_start = None;
                }
            }
            _ => {
                token_start.get_or_insert(i);
            }
        }
    }

    if let Some(start) = token_start {
        tokens.push(input[start..].trim());
    }

    tokens
}



mod tests {
    use std::collections::HashMap;
    use crate::json_reader::JsonValue::{JsonArray, JsonInt, JsonObject, JsonString};
    use super::*;

    #[test]
    fn read_empty_json() {
        let json = "{}";
        let result = read_helper(&json);
        assert!(result.is_none());
    }

    #[test]
    fn read_basic_to_map () {
        let json = r#"{ "key" : "value" }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonString("value".to_string()));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_to_map_long_string () {
        let json = r#"{ "key" : "value is value" }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonString("value is value".to_string()));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_multiple_to_map () {
        let json = r#"{ "key1" : "value1" , "key2" : "value2" }"#; // need the case where its value,
        let mut expected = HashMap::new();
        expected.insert("key1".to_string() , JsonString("value1".to_string()));
        expected.insert("key2".to_string() , JsonString("value2".to_string()));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_multiple_to_map_no_space () {
        let json = r#"{ "key1" : "value1", "key2" : "value2"}"#; // need the case where its value,
        let mut expected = HashMap::new();
        expected.insert("key1".to_string() , JsonString("value1".to_string()));
        expected.insert("key2".to_string() , JsonString("value2".to_string()));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_to_map_with_number () {
        let json = r#"{ "key" : 5 }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonInt(5));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_to_map_with_float () {
        let json = r#"{ "key" : 5.0 }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonFloat(5.0));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_basic_to_map_with_bool () {
        let json = r#"{ "key1" : true, "key2" : false }"#;
        let mut expected = HashMap::new();
        expected.insert("key1".to_string() , JsonBoolean(true));
        expected.insert("key2".to_string() , JsonBoolean(false));
        let result = read_helper(&json);
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
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_json_with_inner_obj () {
        let json = r#"{ "key" : '{ "key1" : true, "key2" : false, "key3" : 5.0 , "key4" : 5, "key5" : "value5" }' }"#;
        let mut expected = HashMap::new();
        let mut expected_inner = HashMap::new();
        expected_inner.insert("key1".to_string() , JsonBoolean(true));
        expected_inner.insert("key2".to_string() , JsonBoolean(false));
        expected_inner.insert("key3".to_string() , JsonFloat(5.0));
        expected_inner.insert("key4".to_string() , JsonInt(5));
        expected_inner.insert("key5".to_string() , JsonString("value5".to_string()));
        expected.insert("key".to_string() , JsonObject(expected_inner));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_json_inner_array () {
        let json = r#"{ "key" : '["value1", "value2", "value3"]' }"#;
        let mut expected = HashMap::new();
        expected.insert("key".to_string() , JsonArray(vec![
            JsonString("value1".to_string()),
            JsonString("value2".to_string()),
            JsonString("value3".to_string()),
        ]));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_json_inner_array_different_types_no_obj () {
        let json = r#"{ "key" : '["value1", 67, 6.7, true, false]'}"#;
        let mut expected = HashMap::new();
        let mut expected_inner = HashMap::new();
        expected_inner.insert("key1".to_string() , JsonBoolean(true));
        expected_inner.insert("key2".to_string() , JsonBoolean(false));
        expected_inner.insert("key3".to_string() , JsonFloat(5.0));
        expected_inner.insert("key4".to_string() , JsonInt(5));
        expected_inner.insert("key5".to_string() , JsonString("value5".to_string()));
        expected.insert("key".to_string() , JsonArray(vec![
            JsonString("value1".to_string()),
            JsonInt(67),
            JsonFloat(6.7),
            JsonBoolean(true),
            JsonBoolean(false)
        ]));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn read_json_inner_array_with_obj () {
        let json = r#"{ "key" : '['{ "key1" : true, "key2" : false, "key3" : 5.0 , "key4" : 5, "key5" : "value5" }']'}"#;
        let mut expected = HashMap::new();
        let mut expected_inner = HashMap::new();
        expected_inner.insert("key1".to_string() , JsonBoolean(true));
        expected_inner.insert("key2".to_string() , JsonBoolean(false));
        expected_inner.insert("key3".to_string() , JsonFloat(5.0));
        expected_inner.insert("key4".to_string() , JsonInt(5));
        expected_inner.insert("key5".to_string() , JsonString("value5".to_string()));
        expected.insert("key".to_string() , JsonArray(vec![
            JsonObject(expected_inner)
        ]));
        let result = read_helper(&json);
        assert_eq!(expected, result.unwrap());
    }
}