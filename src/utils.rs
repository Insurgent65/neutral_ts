
use serde_json::Value;
use crate::constants::*;

/// Merges two JSON schemas represented as `serde_json::Value`.
///
/// This function performs a recursive merge between two JSON objects.
/// If an object has common keys, the values are merged recursively.
/// If the value is not an object, it is directly overwritten.
///
/// # Arguments
///
/// * `a` - A mutable reference to the first JSON object (`serde_json::Value::Object`).
/// * `b` - A reference to the second JSON object (`serde_json::Value::Object`) that will be merged with the first.
///
/// # Example
///
/// ```plaintext
/// use serde_json::{json, Value};
///
/// let mut schema1 = json!({
///     "name": "John",
///     "age": 30,
/// });
///
/// let schema2 = json!({
///     "age": 31,
///     "city": "New York"
/// });
///
/// merge_schema(&mut schema1, &schema2);
/// assert_eq!(schema1, json!({
///     "name": "John",
///     "age": 31,
///     "city": "New York"
/// }));
/// ```
pub fn merge_schema(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(ref mut a_map), Value::Object(ref b_map)) => {
            for (k, v) in b_map {
                if let Some(va) = a_map.get_mut(k) {
                    merge_schema(va, v);
                } else {
                    a_map.insert(k.clone(), v.clone());
                }
            }
        }
        (a, b) => *a = b.clone(),
    }
}

/// Extract same level blocks positions.
///
/// ```plaintext
///
///                  .-----> .-----> {:code:
///                  |       |           {:code: ... :}
///                  |       |           {:code: ... :}
///                  |       |           {:code: ... :}
///  Level block --> |       ·-----> :}
///                  |        -----> {:code: ... :}
///                  |       .-----> {:code:
///                  |       |           {:code: ... :}
///                  ·-----> ·-----> :}
///
/// # Arguments
///
/// * `raw_source` - A string slice containing the template source text.
///
/// # Returns
///
/// * `Ok(Vec<(usize, usize)>)`: A vector of tuples representing the start and end positions of each extracted block.
/// * `Err(usize)`: An error position if there are unmatched closing tags or other issues
/// ```
pub fn extract_blocks(raw_source: &str) -> Result<Vec<(usize, usize)>, usize> {
    let mut blocks = Vec::new();
    let bytes = raw_source.as_bytes();
    let mut curr_pos: usize = 0;
    let mut open_pos: usize;
    let mut nested = 0;
    let mut nested_comment = 0;
    let len_open = BIF_OPEN_B.len();
    let len_close = BIF_CLOSE_B.len();
    let len_src = bytes.len();

    while let Some(pos) = find_bytes(&bytes, BIF_OPEN_B, curr_pos) {
        curr_pos = pos + len_open;
        open_pos = pos;

        // It is important to extract the comments first because they may have bif commented,
        // we avoid that they are detected as valid and other errors.
        if bytes[curr_pos] == BIF_COMMENT_B {
            while let Some(pos) = find_bytes(&bytes, BIF_DELIM_B, curr_pos) {
                curr_pos = pos;

                if curr_pos >= len_src {
                    break;
                }

                if bytes[curr_pos - 1] == BIF_OPEN0 && bytes[curr_pos + 1] == BIF_COMMENT_B  {
                    nested_comment += 1;
                    curr_pos += 1;
                    continue;
                }
                if nested_comment > 0 && bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                    nested_comment -= 1;
                    curr_pos += 1;
                    continue;
                }
                if bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                    curr_pos += len_close;
                    blocks.push((open_pos, curr_pos));
                    break;
                } else {
                    curr_pos += 1;
                }
            }

            continue;
        }

        while let Some(pos) = find_bytes(&bytes, BIF_DELIM_B, curr_pos) {
            curr_pos = pos;

            if curr_pos >= len_src {
                break;
            }

            if bytes[curr_pos - 1] == BIF_OPEN0 {
                nested += 1;
                curr_pos += 1;
                continue;
            }
            if nested > 0 && bytes[curr_pos + 1] == BIF_CLOSE1 {
                nested -= 1;
                curr_pos += 1;
                continue;
            }
            if bytes[curr_pos + 1] == BIF_CLOSE1 {
                curr_pos += len_close;
                blocks.push((open_pos, curr_pos));
                break;
            } else {
                curr_pos += 1;
            }
        }
    }

    // Search BIF_CLOSE in the blocks that are not bif, given that we start looking
    // for BIF_OPEN all these keys are found, if anything is left is BIF_CLOSE
    let mut prev_end = 0;
    for (start, end) in &blocks {
        if let Some(error_pos) = find_bytes(&bytes[prev_end..*start], BIF_CLOSE_B, 0) {
            return Err(error_pos + prev_end);
        }
        prev_end = *end;
    }

    let rest = if curr_pos == 0 { 0 } else { curr_pos - 1 };
    if let Some(error_pos) = find_bytes(&bytes, BIF_CLOSE_B, rest) {
        return Err(error_pos);
    }

    Ok(blocks)
}

fn find_bytes(bytes: &[u8], substring: &[u8], start_pos: usize) -> Option<usize> {
    let bytes_len = bytes.len();
    let subs_len = substring.len();

    if start_pos >= bytes_len || substring.is_empty() || start_pos + subs_len > bytes_len  {
        return None;
    }

    for i in start_pos..=bytes_len.saturating_sub(subs_len) {
        if &bytes[i..i + subs_len] == substring {
            return Some(i);
        }
    }

    None
}

/// Removes a prefix and suffix from a string slice.
///
/// # Arguments
///
/// * `str`: The input string slice.
/// * `prefix`: The prefix to remove.
/// * `suffix`: The suffix to remove.
///
/// # Returns
///
/// * A new string slice with the prefix and suffix removed, or the original string if not found.
pub fn strip_prefix_suffix<'a>(str: &'a str, prefix: &'a str, suffix: &'a str) -> &'a str {
    let start = match str.strip_prefix(prefix) {
        Some(striped) => striped,
        None => return str,
    };
    let end = match start.strip_suffix(suffix) {
        Some(striped) => striped,
        None => return str,
    };

    end
}

/// Retrieves a value from a JSON schema using a specified key.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to retrieve the value from the schema.
///
/// # Returns
///
/// * A `String` containing the retrieved value, or an empty string if the key is not found.
pub fn get_from_key(schema: &Value, key: &str) -> String {
    let tmp: String = format!("{}{}", "/", key);
    let k = tmp.replace(BIF_ARRAY, "/");
    let mut result = "";
    let num: String;

    if let Some(v) = schema.pointer(&k) {
        match v {
            Value::Null => result = "",
            Value::Bool(_b) => result = "",
            Value::Number(n) => {
                num = n.to_string();
                result = num.as_str();
            }
            Value::String(s) => result = s,
            _ => result = "",
        }
    }

    result.to_string()
}

/// Checks if the value associated with a key in the schema is considered empty.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is considered empty, otherwise `false`.
pub fn is_empty_key(schema: &Value, key: &str) -> bool {
    let tmp: String = format!("{}{}", "/", key);
    let k = tmp.replace(BIF_ARRAY, "/");

    if let Some(value) = schema.pointer(&k) {
        match value {
            Value::Object(map) => map.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::String(s) => s.is_empty(),
            Value::Null => true,
            Value::Number(_) => false,
            Value::Bool(_) => false,
        }
    } else {
        true
    }
}

/// Checks if the value associated with a key in the schema is considered a boolean true.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is considered a boolean true, otherwise `false`.
pub fn is_bool_key(schema: &Value, key: &str) -> bool {
    let tmp: String = format!("{}{}", "/", key);
    let k = tmp.replace(BIF_ARRAY, "/");

    if let Some(value) = schema.pointer(&k) {
        match value {
            Value::Object(obj) => !obj.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::String(s) if s.is_empty() || s == "false" => false,
            Value::String(s) => s.parse::<f64>().ok().map_or(true, |n| n > 0.0),
            Value::Null => false,
            Value::Number(n) => n.as_f64().map_or(false, |f| f > 0.0),
            Value::Bool(b) => *b,
        }
    } else {
        false
    }
}

/// Checks if the value associated with a key in the schema is considered an array.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is an array, otherwise `false`.
pub fn is_array_key(schema: &Value, key: &str) -> bool {
    let tmp: String = format!("{}{}", "/", key);
    let k = tmp.replace(BIF_ARRAY, "/");

    if let Some(value) = schema.pointer(&k) {
        match value {
            Value::Object(_) => true,
            Value::Array(_) => true,
            _ => false,
        }
    } else {
        false
    }
}

/// Checks if the value associated with a key in the schema is considered defined.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is defined and not null, otherwise `false`.
pub fn is_defined_key(schema: &Value, key: &str) -> bool {
    let tmp: String = format!("{}{}", "/", key);
    let k = tmp.replace(BIF_ARRAY, "/");

    match schema.pointer(&k) {
        Some(value) => !value.is_null(),
        None => false,
    }
}

/// Finds the position of the first occurrence of BIF_CODE_B in the source string,
/// but only when it is not inside any nested brackets.
///
/// ```plaintext
///                   .------------------------------> params
///                   |       .----------------------> this
///                   |       |
///                   |       |                 .----> code
///                   |       |                 |
///                   v       v                 v
///              ------------ -- ------------------------------
///  {:!snippet; snippet_name >> <div>... {:* ... *:} ...</div> :}
pub fn get_code_position(src: &str) -> Option<usize> {
    let mut level = 0;
    src.as_bytes()
        .windows(2)
        .enumerate()
        .find(|&(_, window)| match window {
            x if x == BIF_OPEN_B => {
                level += 1;
                false
            }
            x if x == BIF_CLOSE_B => {
                level -= 1;
                false
            }
            x if x == BIF_CODE_B && level == 0 => true,
            _ => false,
        })
        .map(|(i, _)| i)
}

/// Removes comments from the template source.
pub fn remove_comments(raw_source: &str) -> String {
    let mut result = String::new();
    let mut blocks = Vec::new();
    let bytes = raw_source.as_bytes();
    let mut curr_pos: usize = 0;
    let mut open_pos: usize;
    let mut nested_comment = 0;
    let len_open = BIF_OPEN_B.len();
    let len_close = BIF_CLOSE_B.len();
    let len_src = bytes.len();

    while let Some(pos) = find_bytes(&bytes, BIF_COMMENT_OPEN_B, curr_pos) {
        curr_pos = pos + len_open;
        open_pos = pos;

        while let Some(pos) = find_bytes(&bytes, BIF_DELIM_B, curr_pos) {
            curr_pos = pos;

            if curr_pos >= len_src {
                break;
            }

            if bytes[curr_pos - 1] == BIF_OPEN0 && bytes[curr_pos + 1] == BIF_COMMENT_B  {
                nested_comment += 1;
                curr_pos += 1;
                continue;
            }
            if nested_comment > 0 && bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                nested_comment -= 1;
                curr_pos += 1;
                continue;
            }
            if bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                curr_pos += len_close;
                blocks.push((open_pos, curr_pos));
                break;
            } else {
                curr_pos += 1;
            }
        }

    }

    let mut prev_end = 0;
    for (start, end) in &blocks {
        result.push_str(&raw_source[prev_end..*start]);
        prev_end = *end;
    }
    result.push_str(&raw_source[curr_pos..]);

    result
}

/// Performs a wildcard matching between a text and a pattern.
///
/// Used in bif "allow" and "declare"
///
/// # Arguments
///
/// * `text`: The text to match against the pattern.
/// * `pattern`: The pattern containing wildcards ('.', '?', '*', '~').
///
/// # Returns
///
/// * `true` if the text matches the pattern, otherwise `false`.
pub fn wildcard_match(text: &str, pattern: &str) -> bool {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    fn match_recursive(text: &[char], pattern: &[char]) -> bool {
        if pattern.is_empty() {
            return text.is_empty();
        }

        let first_char = *pattern.first().unwrap();
        let rest_pattern = &pattern[1..];

        match first_char {
            '\\' => {
                if rest_pattern.is_empty() || text.is_empty() {
                    return false;
                }
                let escaped_char = rest_pattern.first().unwrap();
                match_recursive(&text[1..], &rest_pattern[1..]) && *text.first().unwrap() == *escaped_char
            }
            '.' => {
                match_recursive(text, rest_pattern) || (!text.is_empty() && match_recursive(&text[1..], rest_pattern))
            }
            '?' => {
                !text.is_empty() && match_recursive(&text[1..], rest_pattern)
            }
            '*' => {
                match_recursive(text, rest_pattern) || (!text.is_empty() && match_recursive(&text[1..], pattern))
            }
            '~' => {
                text.is_empty()
            },
            _ => {
                if text.is_empty() || first_char != *text.first().unwrap() {
                    false
                } else {
                    match_recursive(&text[1..], rest_pattern)
                }
            }
        }
    }

    match_recursive(&text_chars, &pattern_chars)
}


/// Finds the position of a tag in the text.
///
/// It is used in the bif "moveto".
///
/// # Arguments
///
/// * `text`: The text to search for the tag.
/// * `tag`: The tag to find.
///
/// # Returns
///
/// * `Some(usize)`: The position of the end of the tag, or None if the tag is not found.
pub fn find_tag_position(text: &str, tag: &str) -> Option<usize> {
    if let Some(start_pos) = text.find(tag) {
        if !tag.starts_with("</") {
            if let Some(end_tag_pos) = text[start_pos..].find('>') {
                return Some(start_pos + end_tag_pos + 1);
            }
        } else {
            return Some(start_pos);
        }
    }

    None
}
