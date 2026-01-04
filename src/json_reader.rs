use std::collections::HashMap;
use crate::json_reader::JsonValue::{JsonArray, JsonBoolean, JsonFloat, JsonInt, JsonObject, JsonString};
use crate::json_reader::Token::{COLON, COMMA, FALSE, LBRACE, LBRACKET, NULL, NUMBER, RBRACE, RBRACKET, STRING, TRUE};

enum Token {
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    STRING(String),
    NUMBER(f64),
    COLON,
    COMMA,
    TRUE,
    FALSE,
    NULL,
    EOF,
    NONE
}

struct Lexer {
    current_index: usize,
    current_token: Token,
    tokens: Vec<Token>,
    json: String
}

impl Lexer {
    pub fn new(json: &str) -> Self {
        Self {
            current_index: 0,
            current_token: Token::NONE,
            tokens: vec![],
            json: json.to_string(),
        }
    }

    pub fn get_tokens(mut self) -> Vec<Token> {
        while self.current_index < self.json.len() {

            if let Some(c) = self.json.chars().nth(self.current_index) {
                match self.current_token {
                    Token::NONE => {
                        match c {
                            '{' => {
                                &self.tokens.push(LBRACE);
                            },
                            '}' => {
                                &self.tokens.push(RBRACE);
                            },
                            '[' => {
                                &self.tokens.push(LBRACKET);
                            },
                            ']' => {
                                &self.tokens.push(RBRACKET);
                            },
                            ':' => {
                                &self.tokens.push(COLON);
                            },
                            ',' => {
                                &self.tokens.push(COMMA);
                            },
                            '"' => {
                                self.current_token = STRING("\"".to_string());
                            },
                            _ => {
                                if (c == 't') {
                                    self.tokens.push(TRUE)
                                } else if (c == 'f') {
                                    self.tokens.push(FALSE)
                                } else if (c == 'n') {
                                    self.tokens.push(NULL)
                                } // else if is number
                            }
                        }
                    }
                    LBRACE | RBRACE | LBRACKET | RBRACKET |
                    COLON | COMMA | TRUE | FALSE | Token::EOF | NULL => {
                        continue
                    },
                    NUMBER(num) => {
                        if (c == ' ') {
                            self.tokens.push(NUMBER(num));
                            continue;
                        }
                        let mut num_string = num.to_string();
                        num_string.push(c);
                        if let Ok(new_num) = num_string.as_str().parse::<f64>() {
                            self.current_token = NUMBER(new_num);
                        }
                    },
                    STRING(text) => {
                        if (c == ' ') {
                            self.tokens.push(STRING(text));
                            continue;
                        }
                    },
                }
            }
        }

        self.tokens
    }
}

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
    return todo!("Doing")
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
    (s.starts_with("'{") && s.ends_with("}'")) || (s.starts_with("{") && s.ends_with("}"))
}

fn is_json_arr(s: &str) -> bool {
    s.starts_with("[") && s.ends_with("]")
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
        let json = r#"{ "key" : { "key1" : true, "key2" : false, "key3" : 5.0 , "key4" : 5, "key5" : "value5" } }"#;
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
        let json = r#"{ "key" : ["value1", "value2", "value3"] }"#;
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
        let json = r#"{ "key" : ["value1", 67, 6.7, true, false]}"#;
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
        let json = r#"{ "key" : [{ "key1" : true, "key2" : false, "key3" : 5.0 , "key4" : 5, "key5" : "value5" }]}"#;
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