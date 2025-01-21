
use neutralts::constants::*;
use neutralts::utils::*;
use serde_json::Value;
use serde_json::json;

const HTML_SOURCE: &str = r#"<!DOCTYPE html>
<html lang="{:lang;:}">
    <head>
        {:*
            comment
        *:}
        <title>{:trans; Site title :}</title>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        {:snippet; current-theme:head :}
        <link rel="stylesheet" href="bootstrap.min.css">
    </head>
    <body class="{:;body-class:}">
        {:snippet; current-theme:body_begin  :}
        {:snippet; current-theme:body-content :}
        {:snippet; current-theme:body-footer  :}
        <script src="jquery.min.js"></script>
    </body>
</html>"#;

#[test]
fn test_extract_blocks_from_html() {
    let expected = vec![
        (28, 37),   // {:lang;:}
        (59, 94),   // {:* comment *:}
        (110, 132), // {:trans; Site title :}
        (257, 289), // {:snippet; current-theme:head :}
        (376, 391), // {:;body-class:}
        (402, 441), // {:snippet; current-theme:body_begin  :}
        (450, 490), // {:snippet; current-theme:body-content :}
        (499, 539), // {:snippet; current-theme:body-footer  :}
    ];
    assert_eq!(extract_blocks(HTML_SOURCE).unwrap(), expected);
}

#[test]
fn test_merge_schema() {
    let mut a: Value = serde_json::json!({
        "name": "John",
        "age": 30,
        "address": {
            "city": "New York"
        }
    });

    let b: Value = serde_json::json!({
        "age": 25,
        "email": "john@example.com",
        "address": {
            "street": "123 Main St"
        }
    });

    merge_schema(&mut a, &b);

    let expected: Value = serde_json::json!({
        "name": "John",
        "age": 25,
        "email": "john@example.com",
        "address": {
            "city": "New York",
            "street": "123 Main St"
        }
    });

    assert_eq!(a, expected);
}

#[test]
fn test_merge_schema_nested() {
    let mut a: Value = serde_json::json!({
        "user": {
            "name": "John",
            "details": {
                "age": 30,
                "address": {
                    "city": "New York"
                }
            }
        }
    });

    let b: Value = serde_json::json!({
        "user": {
            "details": {
                "age": 25,
                "address": {
                    "street": "123 Main St"
                }
            }
        }
    });

    merge_schema(&mut a, &b);

    let expected: Value = serde_json::json!({
        "user": {
            "name": "John",
            "details": {
                "age": 25,
                "address": {
                    "city": "New York",
                    "street": "123 Main St"
                }
            }
        }
    });

    assert_eq!(a, expected);
}

#[test]
fn test_merge_schema_non_object() {
    let mut a: Value = serde_json::json!(42);
    let b: Value = serde_json::json!("hello");

    merge_schema(&mut a, &b);

    let expected: Value = serde_json::json!("hello");

    assert_eq!(a, expected);
}

#[test]
fn test_strip_prefix_suffix() {
    let str = "{:defined; name >> hello :}";
    let prefix = BIF_OPEN;
    let suffix = BIF_CLOSE;

    let expected = "defined; name >> hello ";
    assert_eq!(strip_prefix_suffix(str, prefix, suffix), expected);
}

#[test]
fn test_get_from_key() {
    let schema = json!({
        "name": "John",
        "age": 30,
    });

    assert_eq!(get_from_key(&schema, "name"), "John");
    assert_eq!(get_from_key(&schema, "age"), "30");
}

#[test]
fn test_is_empty_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert_eq!(is_empty_key(&schema, "true"), false);
    assert_eq!(is_empty_key(&schema, "false"), false);
    assert_eq!(is_empty_key(&schema, "hello"), false);
    assert_eq!(is_empty_key(&schema, "zero"), false);
    assert_eq!(is_empty_key(&schema, "one"), false);
    assert_eq!(is_empty_key(&schema, "spaces"), false);
    assert_eq!(is_empty_key(&schema, "empty"), true);
    assert_eq!(is_empty_key(&schema, "null"), true);
    assert_eq!(is_empty_key(&schema, "emptyarr"), true);
    assert_eq!(is_empty_key(&schema, "array/true"), false);
    assert_eq!(is_empty_key(&schema, "array/false"), false);
    assert_eq!(is_empty_key(&schema, "array/hello"), false);
    assert_eq!(is_empty_key(&schema, "array/zero"), false);
    assert_eq!(is_empty_key(&schema, "array/one"), false);
    assert_eq!(is_empty_key(&schema, "array/spaces"), false);
    assert_eq!(is_empty_key(&schema, "array/empty"), true);
    assert_eq!(is_empty_key(&schema, "array/null"), true);
    assert_eq!(is_empty_key(&schema, "non_existent_key"), true);
}


#[test]
fn test_is_bool_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert_eq!(is_bool_key(&schema, "true"), true);
    assert_eq!(is_bool_key(&schema, "false"), false);
    assert_eq!(is_bool_key(&schema, "hello"), true);
    assert_eq!(is_bool_key(&schema, "zero"), false);
    assert_eq!(is_bool_key(&schema, "one"), true);
    assert_eq!(is_bool_key(&schema, "spaces"), true);
    assert_eq!(is_bool_key(&schema, "empty"), false);
    assert_eq!(is_bool_key(&schema, "null"), false);
    assert_eq!(is_bool_key(&schema, "emptyarr"), false);
    assert_eq!(is_bool_key(&schema, "array/true"), true);
    assert_eq!(is_bool_key(&schema, "array/false"), false);
    assert_eq!(is_bool_key(&schema, "array/hello"), true);
    assert_eq!(is_bool_key(&schema, "array/zero"), false);
    assert_eq!(is_bool_key(&schema, "array/one"), true);
    assert_eq!(is_bool_key(&schema, "array/spaces"), true);
    assert_eq!(is_bool_key(&schema, "array/empty"), false);
    assert_eq!(is_bool_key(&schema, "array/null"), false);
    assert_eq!(is_bool_key(&schema, "non_existent_key"), false);
}

#[test]
fn test_is_array_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert_eq!(is_array_key(&schema, "true"), false);
    assert_eq!(is_array_key(&schema, "false"), false);
    assert_eq!(is_array_key(&schema, "hello"), false);
    assert_eq!(is_array_key(&schema, "zero"), false);
    assert_eq!(is_array_key(&schema, "one"), false);
    assert_eq!(is_array_key(&schema, "spaces"), false);
    assert_eq!(is_array_key(&schema, "empty"), false);
    assert_eq!(is_array_key(&schema, "null"), false);
    assert_eq!(is_array_key(&schema, "emptyarr"), true);
    assert_eq!(is_array_key(&schema, "array/true"), false);
    assert_eq!(is_array_key(&schema, "array/false"), false);
    assert_eq!(is_array_key(&schema, "array/hello"), false);
    assert_eq!(is_array_key(&schema, "array/zero"), false);
    assert_eq!(is_array_key(&schema, "array/one"), false);
    assert_eq!(is_array_key(&schema, "array/spaces"), false);
    assert_eq!(is_array_key(&schema, "array/empty"), false);
    assert_eq!(is_array_key(&schema, "array/null"), false);
    assert_eq!(is_array_key(&schema, "non_existent_key"), false);
}

#[test]
fn test_is_defined_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert_eq!(is_defined_key(&schema, "true"), true);
    assert_eq!(is_defined_key(&schema, "false"), true);
    assert_eq!(is_defined_key(&schema, "hello"), true);
    assert_eq!(is_defined_key(&schema, "zero"), true);
    assert_eq!(is_defined_key(&schema, "one"), true);
    assert_eq!(is_defined_key(&schema, "spaces"), true);
    assert_eq!(is_defined_key(&schema, "empty"), true);
    assert_eq!(is_defined_key(&schema, "null"), false);
    assert_eq!(is_defined_key(&schema, "emptyarr"), true);
    assert_eq!(is_defined_key(&schema, "array/true"), true);
    assert_eq!(is_defined_key(&schema, "array/false"), true);
    assert_eq!(is_defined_key(&schema, "array/hello"), true);
    assert_eq!(is_defined_key(&schema, "array/zero"), true);
    assert_eq!(is_defined_key(&schema, "array/one"), true);
    assert_eq!(is_defined_key(&schema, "array/spaces"), true);
    assert_eq!(is_defined_key(&schema, "array/empty"), true);
    assert_eq!(is_defined_key(&schema, "array/null"), false);
    assert_eq!(is_defined_key(&schema, "non_existent_key"), false);
}

#[test]
fn test_get_code_position() {
    let src = r#"!snippet; {:defined; name >> snippet_name :}{:else: none :} >> <div>... {:* comment *:} ...</div> "#;
    assert_eq!(get_code_position(src), Some(60));
}

#[test]
fn test_wildcard_match() {
    // Basic match
    assert!(wildcard_match("hello", "hello"));

    // Wildcard '*' matches any sequence of characters
    assert!(wildcard_match("hello", "*"));
    assert!(wildcard_match("hello", "h*o"));
    assert!(wildcard_match("hello", "he*llo"));
    assert!(wildcard_match("hello", "hell*"));
    assert!(wildcard_match("hello", "*hello"));

    // Wildcard '?' matches any single character
    assert!(wildcard_match("hello", "h?llo"));
    assert!(wildcard_match("hello", "?ello"));
    assert!(wildcard_match("hello", "he?lo"));
    assert!(wildcard_match("hello", "hell?"));
    assert!(wildcard_match("hello", "*ell?"));

    // Mixed usage of '*' and '?'
    assert!(wildcard_match("hello", "h?*o"));
    assert!(wildcard_match("hello", "h*ll?"));
    assert!(wildcard_match("hello", "?*llo"));
    assert!(wildcard_match("hello", "he*l?"));

    // Escaping special characters
    assert!(wildcard_match("hell*o", "hell\\*o"));
    assert!(wildcard_match("hell.o", "hell\\.o"));
    assert!(wildcard_match("hell?o", "hell\\?o"));

    // Empty pattern
    assert!(wildcard_match("", ""));

    // Special character '~' matches empty string
    assert!(wildcard_match("", "~"));
}


#[test]
fn test_find_tag_position() {
    // Basic match for opening tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<html"), Some(39));

    // Match for closing tag
    assert_eq!(find_tag_position(HTML_SOURCE, "</html"), Some(598));

    // Nested tags
    assert_eq!(find_tag_position(HTML_SOURCE, "<head"), Some(50));
    assert_eq!(find_tag_position(HTML_SOURCE, "</head"), Some(351));

    // Match for meta tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<meta"), Some(171));

    // Non-existent tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<nonexistent>"), None);

    // Self-closing tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<link"), Some(346));

    // Empty string source
    assert_eq!(find_tag_position("", "<html"), None);
}
