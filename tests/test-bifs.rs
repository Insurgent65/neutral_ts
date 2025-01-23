use neutralts::constants::*;

// requires "preserve_order": serde_json = { version = "1.0", features = ["preserve_order"] }

// Here you can add things but you cannot modify or remove them.
const SCHEMA: &str = r#"{
    "config": {
        "infinite_loop_max_bifs": 555000,
        "comments": "remove",
        "errors": "hide"
    },
    "inherit": {
        "snippets": {
            "__hello-nts": "<div>{:trans; ref:greeting-nts :}</div>"
        },
        "declare": {
            "any": "*",
            "traversal": "/* \\\\* *\\.\\.*",
            "_test-nts": "en es fr de nts",
            "_test-nts-empty": "~ nts en es fr de",
            "_test-nts-asterisk": "*en* nts es fr de",
            "_test-nts-question": "en?nts nts es fr de",
            "_test-nts-dot": "en.nts es fr de"
        },
        "params": {},
        "locale": {
            "current": "en",
            "trans": {
                "en": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "en-US": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "en-UK": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "es": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                },
                "es-ES": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                },
                "de": {
                    "Hello nts": "Hallo",
                    "ref:greeting-nts": "Hallo"
                },
                "fr": {
                    "Hello nts": "Bonjour",
                    "ref:greeting-nts": "Bonjour"
                },
                "el": {
                    "Hello nts": "Γεια σας",
                    "ref:greeting-nts": "Γεια σας"
                }
            }
        }
    },
    "data": {
        "__hello-nts": "Hello nts",
        "__ref-hello-nts": "__hello-nts",
        "__test-local": "local",
        "__test-nts": "nts",
        "__test-empty-nts": "",
        "__test-null-nts": null,
        "__test-zero-nts": 0,
        "__test-bool-true-string-nts": true,
        "__test-bool-true-num-nts": 1,
        "__test-bool-false-string-nts": false,
        "__test-bool-false-num-nts": 0,
        "__test-bool-false-empty-nts": "",
        "__test-arr-nts": [
            "one",
            "two",
            "three"
        ],
        "__test-arr-empty-nts": [],
        "__test-obj-empty-nts": {},
        "__test-obj-nts": {
            "level1": "Ok",
            "level1-obj": {
                "level1": "Ok",
                "level2-obj": {
                    "level2": "Ok",
                    "level3-arr": [
                        "one",
                        "two",
                        "three"
                    ]
                }
            }
        }
    }
}"#;

// SOME BIF DEPENDS ON OTHERS, IF IT FAILS CHECK THESE DEPENDENCIES.

#[test]
fn test_bif_neutral() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:neutral; template >> system :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>{:neutral; template >> system :}</div>");
}

#[test]
fn test_bif_unknown() {
    let schema = r#"
    {
        "config": {
            "comments": "keep"
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:unk;:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_comments() {
    let schema = r#"
    {
        "config": {
            "comments": "keep"
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:* comment *:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_comments_multiline() {
    let schema = r#"
    {
        "config": {
            "comments": "keep"
        }
    }
    "#
    .trim();
    let source = r#"
        {:*

            test comment

        *:}
        <div></div>
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_comments_nested() {
    let schema = r#"
    {
        "config": {
            "comments": "keep"
        }
    }
    "#
    .trim();
    let source = r#"
        {:*
            test comment
            {:* comment *:}
        *:}
        <div></div>
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_comments_complex() {
    let schema = r#"
    {
        "config": {
            "comments": "keep"
        }
    }
    "#
    .trim();
    let source = r#"
        {:* comment *:}
        {:* {:code; *:}
            {:code;
                {:* comment *:}
                <div>{:; {:* comment *:} __test-nts {:* comment *:} :}</div>
            :}
        {:* :} *:}
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_comments_remove() {
    let schema = r#"
    {
        "config": {
            "comments": "remove"
        }
    }
    "#
    .trim();
    let source = r#"
        {:* comment *:}
        {:* {:code; *:}
            {:code;
                {:* comment *:}
                <div>{:; {:* comment *:} __test-nts {:* comment *:} :}</div>
            :}
        {:* :} *:}
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_unprintable() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_unprintable_spaces() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div> {:;:} </div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>  </div>");
}

#[test]
fn test_bif_unprintable_upline() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("|  \n  {:^;:}<div></div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "|<div></div>");
}

#[test]
fn test_bif_unprintable_comments() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:; {:* comment *:} :}<div></div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_var() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_var_arr() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-obj-nts->level1-obj->level2-obj->level3-arr->0:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>one</div>");
}

#[test]
fn test_bif_var_dynamic_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__hello-{:;__test-nts:}:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Hello nts</div>");
}

#[test]
fn test_bif_var_error_dynamic_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;{:;__ref-hello-nts:}:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_var_undefined() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__dfhs76tfwq65dhtw563hjknv__:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_var_upline() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("|  \n  {:^;__test-nts:}<div></div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "|nts<div></div>");
}

#[test]
fn test_bif_var_error_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_var_error_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_trans() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:trans; Hello nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Γεια σας</div>");
}

#[test]
fn test_bif_trans_dynamic_evaluation() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:trans; {:;__hello-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Γεια σας</div>");
}

#[test]
fn test_bif_trans_no_trans() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:trans; This text has no __translation__ :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>This text has no __translation__</div>");
}

#[test]
fn test_bif_trans_no_trans_negate() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:!trans; This text has no __translation__ :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_trans_negate() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:!trans; Hello nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Γεια σας</div>");
}

#[test]
fn test_bif_trans_error_scope() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:+trans; Hello nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_lang() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:lang;:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>el</div>");
}

#[test]
fn test_bif_lang_comment() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:lang; {:* comment *:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>el</div>");
}

#[test]
fn test_bif_lang_error_negate() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:!lang;:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_lang_error_scope() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:+lang;:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_code_empty() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_code_literal() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; Hello :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Hello</div>");
}

#[test]
fn test_bif_code_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_code_flag_safe() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:flg; safe :} >> <div>{:;__test-nts:}</div> :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(
        result,
        "<div>&lt;div&gt;&#123;:;__test-nts:&#125;&lt;&#x2F;div&gt;</div>"
    );
}

#[test]
fn test_bif_code_flag_encode_tags() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template
        .set_src_str("<div>{:code; {:flg; encode_tags :} >> <div>{:;__test-nts:}</div> :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>&lt;div&gt;nts&lt;&#x2F;div&gt;</div>");
}

#[test]
fn test_bif_code_flag_noparse() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:flg; noparse :} >> <div>{:;__test-nts:}</div> :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>{:;__test-nts:}</div></div>");
}

#[test]
fn test_bif_code_flag_encode_tags_after() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:flg; encode_tags_after :} >> <div>{:code; <div>{:;__test-nts:}</div> :}</div> :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(
        result,
        "<div>&lt;div&gt;&lt;div&gt;nts&lt;&#x2F;div&gt;&lt;&#x2F;div&gt;</div>"
    );
}

#[test]
fn test_bif_code_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:include; {:flg; require :} >> tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_code_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+code; {:include; {:flg; require :} >> tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_param() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:param; 1 >> one :} {:param; 1 :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>one</div>");
}

#[test]
fn test_bif_param_set_outside_code() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:param; 1 >> one :}{:param; 1 :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_param_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:param; {:;__test-nts:} >> {:;__test-nts:} :} {:param; {:;__test-nts:} :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_param_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:param; 1 >> one :} {:code; {:param; 1 :} :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>one</div>");
}

#[test]
fn test_bif_param_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:param; {:;__test-nts:} >> {:;__test-nts:} :} :}{:param; {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts >> en :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>en</div>");
}

#[test]
fn test_bif_allow_evaluate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-{:;__test-nts:} >> {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_allow_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts >> notallow :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!allow; traversal >> is not traversal :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is not traversal</div>");
}

#[test]
fn test_bif_allow_negate_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!allow; traversal >> ../istraversal :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_any() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; any >> something :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>something</div>");
}

#[test]
fn test_bif_allow_wildcard_empty() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-empty >>  :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_wildcard_asterisk() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-asterisk >> en-nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>en-nts</div>");
}

#[test]
fn test_bif_allow_wildcard_asterisk_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-asterisk >> not :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_wildcard_dot_1() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-dot >> ennts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>ennts</div>");
}

#[test]
fn test_bif_allow_wildcard_dot_2() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-dot >> en-nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>en-nts</div>");
}

#[test]
fn test_bif_allow_wildcard_dot_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-dot >> not :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_wildcard_question() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-question >> en-nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>en-nts</div>");
}

#[test]
fn test_bif_allow_wildcard_question_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts-question >> ennts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+allow; _test-nts >> notallow :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_flag_partial() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow;{:flg; partial :} _test-nts >> nts and more :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts and more</div>");
}

#[test]
fn test_bif_allow_flag_casein() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow;{:flg; casein :} _test-nts >> NTS :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>NTS</div>");
}

#[test]
fn test_bif_allow_flag_replace() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow;{:flg; replace :} _test-nts >> nts and more :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_allow_flag_noerror() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow;{:flg; noerror :} _test-nts >> nts and more :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_allow_multi_flags() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; {:flg; casein replace :} _test-nts >> NTS :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_declare() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:include; tests/snippets.ntpl :}{:allow; test-for-tests >> one :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>one</div>");
}

#[test]
fn test_bif_declare_no_outside_include() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:declare; test-for-tests >> one two three :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_for() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n 0 9 >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>0123456789</div>");
}

#[test]
fn test_bif_for_rev() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n 9 0 >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>9876543210</div>");
}

#[test]
fn test_bif_for_params_fails_1() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n a b >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_for_params_fails_2() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n a >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_for_params_fails_3() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_for_params_fails_4() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_for_no_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!for; n 0 9 >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_for_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+for; n 0 9 >> {:;n:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_filled() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-nts >> is filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is filled</div>");
}

#[test]
fn test_bif_filled_obj_levels() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-obj-nts->level1-obj->level2-obj->level2 >> {:;__test-obj-nts->level1-obj->level2-obj->level2:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_filled_evaluate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-{:;__test-nts:} >> {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_filled_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!filled; __test-empty-nts >> is not filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is not filled</div>");
}

#[test]
fn test_bif_filled_negate_undefined() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!filled; undefined-var >> is not filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is not filled</div>");
}

#[test]
fn test_bif_filled_obj() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-obj_nts >> is filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is filled</div>");
}

#[test]
fn test_bif_filled_obj_empty_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!filled; __test-obj-empty-nts >> is not filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is not filled</div>");
}

#[test]
fn test_bif_filled_arr() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-arr-nts >> is filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is filled</div>");
}

#[test]
fn test_bif_filled_arr_empty_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!filled; __test-arr-empty-nts >> is not filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is not filled</div>");
}

#[test]
fn test_bif_filled_null_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!filled; __test-null-nts >> is not filled :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>is not filled</div>");
}

#[test]
fn test_bif_filled_zero() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-zero-nts >> if it has 0 it has something :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>if it has 0 it has something</div>");
}

#[test]
fn test_bif_filled_false() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-bool-false-string-nts >> if it has 'false' it has something :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>if it has 'false' it has something</div>");
}

#[test]
fn test_bif_filled_true() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:filled; __test-bool-true-string-nts >> if it has 'true' it has something :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>if it has 'true' it has something</div>");
}

#[test]
fn test_bif_filled_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+filled; __test-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_filled_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; __test-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_bool() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_obj_levels() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-obj-nts->level1-obj->level2-obj->level2 >> {:;__test-obj-nts->level1-obj->level2-obj->level2:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_bool_evaluate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-{:;__test-nts:} >> {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!bool; __test-bool-false-string-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_negate_undefined() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!bool; undefined-var >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_obj() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-obj_nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_obj_empty_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!bool; __test-obj-empty-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_obj_empty() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-obj-empty-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_bool_arr() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-arr-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_arr_empty_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!bool; __test-arr-empty-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_null_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!bool; __test-null-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_zero() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-zero-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_bool_false() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-bool-false-string-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_bool_true() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-bool-true-string-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_bool_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+bool; __test-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_bool_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; __test-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_array() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-obj-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_array_obj_levels() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-obj-nts->level1-obj->level2-obj >> Ok :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_array_evaluate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-obj-{:;__test-nts:} >> {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_array_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!array; __test-obj-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_array_negate_undefined() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!array; undefined-var >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_array_obj_empty() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-obj-empty-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_array_arr() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-arr-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_array_arr_empty() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-arr-empty-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_array_null() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-null-nts >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_array_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+array; __test-obj-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_array_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:array; __test-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_else() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; :}{:else; nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_else_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; 1 :}{:!else; nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>1nts</div>");
}

#[test]
fn test_bif_else_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; :}{:+else; __test-obj-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_else_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; :}{:else; __test-obj-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_snippet() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_snippet_evalueation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet-{:;__test-nts:} :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet nts</div></div>");
}

#[test]
fn test_bif_snippet_nested() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet-nested :}{:snippet; test-snippet-nested-next :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet nested</div></div>");
}

#[test]
fn test_bif_snippet_nested_set_in_code() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template
        .set_src_str("<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet-code :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet code</div></div>");
}

#[test]
fn test_bif_snippet_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:* error, unnecessary scope, it is auto *:}{:+snippet; test-snippet :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_snippet_no_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:!snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_include() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/include-snippets.ntpl :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_include_flag_require() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template
        .set_src_str("<div>{:include; {:flg; require :} >> tests/include-snippets.ntpl :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_include_flag_require_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; {:flg; require :} >> tests/not-found.ntpl :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_include_not_found() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/not-found.ntpl :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_include_allow() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; {:allow; any >> {:;__test-nts:} :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_include_allow_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_include_without_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/include-snippets.ntpl :}{:include; tests/include-snippets.ntpl :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>OkOk</div>");
}

#[test]
fn test_bif_include_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/include-snippets.ntpl :}{:!include; tests/include-snippets.ntpl :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_include_text_files() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; {:flg; safe :} >> tests/include.txt :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(
        result,
        "<div>Lorem Ipsum &lt;div&gt;&#123;:code; :&#125;&lt;&#x2F;div&gt;</div>"
    );
}

#[test]
fn test_bif_include_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:* error, unnecessary scope, it is auto *:}{:+include; tests/include-snippets.ntpl :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:eval; a >> b :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>b</div>");
}

#[test]
fn test_bif_eval_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:eval; {:;__test-nts:} >> {:;__eval__:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_eval_negate_1() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!eval; {:;__test-nts:} >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_eval_negate_2() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!eval; {:;__test-empty-nts:} >> nts :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_eval_scope_1() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:eval; {:include; tests/snippets.ntpl :} >> nts :}{:snippet; test-snippet :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_eval_scope_2() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:+eval; {:include; tests/snippets.ntpl :} >> nts :}{:snippet; test-snippet :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_eval_scope_3() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:+eval; 1 >> {:include; tests/snippets.ntpl :}  :}{:snippet; test-snippet :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_eval_scope_4() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+eval; {:include; tests/snippets.ntpl :} >> {:include; tests/snippets.ntpl :}  :}{:snippet; test-snippet :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_locale() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:locale; tests/locale.es.json :}{:trans; test-locale :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_locale_evaluation() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(
        "<div>{:locale; tests/locale.{:lang;:}.json :}{:trans; Test {:;__test-nts:} :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok nts</div>");
}

#[test]
fn test_bif_locale_flag_inline() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let source = r#"
        {:locale; {:flg; inline :} >>
            {
                "trans": {
                    "{:lang;:}": {
                        "test-locale": "inline"
                    }
                }
            }
        :}
        <div>{:trans; test-locale :}</div>
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>inline</div>");
}

#[test]
fn test_bif_locale_flag_require() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(
        "<div>{:locale; {:flg; require :} >> tests/locale.es.json :}{:trans; test-locale :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_locale_flag_require_fails() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:locale; {:flg; require :} >> tests/not-found.es.json :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_locale_not_found() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:locale; tests/not-found.es.json :}{:trans; test-locale :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>test-locale</div>");
}

#[test]
fn test_bif_locale_allow() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:locale; {:allow; any >> {:;__test-nts:} :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_locale_allow_fails() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:locale; {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_locale_negate() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:!locale; tests/locale.es.json :}{:!locale; tests/locale.es.json :}{:trans; test-locale :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Ok</div>");
}

#[test]
fn test_bif_locale_scope() {
    let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:+locale; tests/locale.es.json :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_flg() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:flg; any :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_each() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:each; __test-obj-nts->level1-obj->level2-obj->level3-arr key value >> {:;key:}={:;value:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>0=one1=two2=three</div>");
}

#[test]
fn test_bif_each_iterate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:code; {:param; array-name >> __test-obj-nts :} {:snippet; iterate-array :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(
        result,
        "<div>\n        level1=Ok\n        level1-obj:\n                level1=Ok\n                level2-obj:\n                        level2=Ok\n                        level3-arr:\n                                0=one\n                                1=two\n                                2=three</div>"
    );
}

#[test]
fn test_bif_coalesce() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:coalesce; {:code; :} {:code; this :} {:code; ... :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>this</div>");
}

#[test]
fn test_bif_coalesce_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:coalesce; {:;__test-empty-nts:} {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_coalesce_nested() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:coalesce; {:code; :} {:coalesce; {:code; :} {:coalesce; {:code; :} {:code; this :} {:code; ... :} :} {:code; ... :} :} {:code; ... :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>this</div>");
}

#[test]
fn test_bif_coalesce_negate_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!coalesce; {:code; :} {:code; this :} {:code; ... :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_coalesce_big() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:snippet; big-coalesce :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>OK1-Ok2-Ok3-Ok4-Ok5-Ok6</div>");
}

#[test]
fn test_bif_coalesce_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:+coalesce; {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div><div>test snippet</div></div>");
}

#[test]
fn test_bif_coalesce_no_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:coalesce; {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_replace() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:replace; /a/b/ >> acbde :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>bcbde</div>");
}

#[test]
fn test_bif_replace_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>Hello one</div>");
}

#[test]
fn test_bif_replace_delim_1() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:replace; |a|b| >> acbde :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>bcbde</div>");
}

#[test]
fn test_bif_replace_delim_2() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:replace; ~a~b~ >> acbde :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>bcbde</div>");
}

#[test]
fn test_bif_replace_delim_3() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:replace; :a:b: >> acbde :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>bcbde</div>");
}

#[test]
fn test_bif_replace_params_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:replace; a/b >> acbde :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_moveto_head() {
    let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; <head >> <script></script> :}
    "#.trim();
    let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head><script></script>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, out);
}

#[test]
fn test_bif_moveto_head_once() {
    let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; <head >> <script></script> :}
        {:moveto; <head >> <script></script> :}
        {:moveto; <head >> <script></script> :}
    "#.trim();
    let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head><script></script>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, out);
}

#[test]
fn test_bif_moveto_head_ends() {
    let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; </head >> <script></script> :}
    "#.trim();
    let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            <script></script></head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, out);
}

#[test]
fn test_bif_moveto_body_ends() {
    let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; </body >> <script></script> :}
    "#.trim();
    let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            <script></script></body>
        </html>
    "#.trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, out);
}

#[test]
fn test_bif_moveto_body() {
    let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; <body >> <script></script> :}
    "#.trim();
    let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body><script></script>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(source);
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, out);
}

#[test]
fn test_bif_date_timestamp() {
    use std::time::SystemTime;
    fn is_timestamp(value: u64) -> bool {
        SystemTime::UNIX_EPOCH
            .checked_add(std::time::Duration::from_secs(value))
            .is_some()
    }
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:date; :}");
    let result = template.render().parse::<u64>().unwrap();

    assert_eq!(template.has_error(), false);
    assert!(is_timestamp(result));
}

#[test]
fn test_bif_date() {
    use chrono::{DateTime, Utc};
    use std::str::FromStr;
    pub fn is_valid_rfc3339(value: &str) -> bool {
        DateTime::<Utc>::from_str(value).is_ok()
    }
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:date; %Y-%m-%d %H:%M:%S :}");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert!(!is_valid_rfc3339(&result));
}

#[test]
fn test_bif_rand() {
    pub fn number(s: &str, x: usize) -> bool {
        s.chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .len()
            == x
    }

    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:rand; :}");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert!(number(&result, 9));
}

#[test]
fn test_bif_rand_10_99() {
    pub fn number(s: &str, x: usize) -> bool {
        s.chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .len()
            == x
    }

    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:rand; 10..99 :}");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert!(number(&result, 2));
}

#[test]
fn test_bif_hash() {
    fn is_md5_like(s: &str) -> bool {
        s.len() == 32 && s.chars().all(|c| c.is_digit(16))
    }

    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:hash; :}");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert!(is_md5_like(&result));
}

#[test]
fn test_bif_hash_evaluate() {
    fn is_md5_like(s: &str) -> bool {
        s.len() == 32 && s.chars().all(|c| c.is_digit(16))
    }

    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:hash; {:;__hello-nts:} :}");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert!(is_md5_like(&result));
}

#[test]
fn test_bif_hash_evaluate_2() {
    use md5::{Digest, Md5};

    pub fn calculate_md5(s: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(s);
        format!("{:x}", hasher.finalize())
    }
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:hash; {:;__test-nts:} :}");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(calculate_md5("nts"), result);
}

#[test]
fn test_bif_data() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:data; tests/local-data.json :}{:;local::hello:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>local hello</div>");
}

#[test]
fn test_bif_data_evaluation() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:data; tests/{:;__test-local:}-data.json :}{:;local::hello-nts:}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_data_flag_require() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str(
        "<div>{:data; {:flg; require :} tests/local-data.json :}{:;local::hello:}</div>",
    );
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div>local hello</div>");
}

#[test]
fn test_bif_data_flag_require_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:data; {:flg; require :} >> tests/not-found.json :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_data_allow() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:data; {:allow; any >> {:;__test-nts:} :} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_data_allow_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:data; {:;__test-nts:} :}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_exit() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>nts");
}

#[test]
fn test_bif_exit_custom_status() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 1600 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "1600");
    assert_eq!(template.get_status_text(), "");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>nts");
}

#[test]
fn test_bif_exit_custom_status_param() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 1600 >> some :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "1600");
    assert_eq!(template.get_status_text(), "");
    assert_eq!(template.get_status_param(), "some");
    assert_eq!(result, "<div>nts");
}

#[test]
fn test_bif_exit_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:!exit; :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>ntsnts</div>");
}

#[test]
fn test_bif_exit_202_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:!exit; 202 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "202");
    assert_eq!(template.get_status_text(), "Accepted");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>ntsnts</div>");
}

#[test]
fn test_bif_exit_206() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 206 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "206");
    assert_eq!(template.get_status_text(), "Partial Content");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>nts");
}

#[test]
fn test_bif_exit_301() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 301 >> /home :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "301");
    assert_eq!(template.get_status_text(), "Moved Permanently");
    assert_eq!(template.get_status_param(), "/home");
    assert_eq!(result, "301 Moved Permanently\n/home");
}

#[test]
fn test_bif_exit_302() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 302 >> /home :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "302");
    assert_eq!(template.get_status_text(), "Found");
    assert_eq!(template.get_status_param(), "/home");
    assert_eq!(result, "302 Found\n/home");
}

#[test]
fn test_bif_exit_303() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 303 >> /home :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "303");
    assert_eq!(template.get_status_text(), "See Other");
    assert_eq!(template.get_status_param(), "/home");
    assert_eq!(result, "303 See Other\n/home");
}

#[test]
fn test_bif_exit_307() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 307 >> /home :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "307");
    assert_eq!(template.get_status_text(), "Temporary Redirect");
    assert_eq!(template.get_status_param(), "/home");
    assert_eq!(result, "307 Temporary Redirect\n/home");
}

#[test]
fn test_bif_exit_308() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 308 >> https://example.com/ :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "308");
    assert_eq!(template.get_status_text(), "Permanent Redirect");
    assert_eq!(template.get_status_param(), "https://example.com/");
    assert_eq!(result, "308 Permanent Redirect\nhttps://example.com/");
}

#[test]
fn test_bif_exit_401() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 401 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "401");
    assert_eq!(template.get_status_text(), "Unauthorized");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "401 Unauthorized");
}

#[test]
fn test_bif_exit_403() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 403 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "403");
    assert_eq!(template.get_status_text(), "Forbidden");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "403 Forbidden");
}

#[test]
fn test_bif_exit_404() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 404 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "404");
    assert_eq!(template.get_status_text(), "Not Found");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "404 Not Found");
}

#[test]
fn test_bif_exit_500() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 500 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "500");
    assert_eq!(template.get_status_text(), "Internal Server Error");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "500 Internal Server Error");
}

#[test]
fn test_bif_exit_503() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:exit; 503 :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "503");
    assert_eq!(template.get_status_text(), "Service Unavailable");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "503 Service Unavailable");
}

#[test]
fn test_bif_redirect_301() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; 301 >> https://example.com/ :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "301");
    assert_eq!(template.get_status_text(), "Moved Permanently");
    assert_eq!(template.get_status_param(), "https://example.com/");
    assert_eq!(result, "301 Moved Permanently\nhttps://example.com/");
}

#[test]
fn test_bif_redirect_302() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; 302 >> https://example.com/ :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "302");
    assert_eq!(template.get_status_text(), "Found");
    assert_eq!(template.get_status_param(), "https://example.com/");
    assert_eq!(result, "302 Found\nhttps://example.com/");
}

#[test]
fn test_bif_redirect_303() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; 303 >> https://example.com/ :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "303");
    assert_eq!(template.get_status_text(), "See Other");
    assert_eq!(template.get_status_param(), "https://example.com/");
    assert_eq!(result, "303 See Other\nhttps://example.com/");
}

#[test]
fn test_bif_redirect_307() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; 307 >> https://example.com/ :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "307");
    assert_eq!(template.get_status_text(), "Temporary Redirect");
    assert_eq!(template.get_status_param(), "https://example.com/");
    assert_eq!(result, "307 Temporary Redirect\nhttps://example.com/");
}

#[test]
fn test_bif_redirect_308() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; 308 >> https://example.com/ :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "308");
    assert_eq!(template.get_status_text(), "Permanent Redirect");
    assert_eq!(template.get_status_param(), "https://example.com/");
    assert_eq!(result, "308 Permanent Redirect\nhttps://example.com/");
}

#[test]
fn test_bif_redirect_js_reload_top() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:top :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "js:reload:top");
    assert_eq!(result, REDIR_JS_RELOAD_TOP);
}

#[test]
fn test_bif_redirect_js_reload_top_param() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:top >> some :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "some");
    assert_eq!(result, REDIR_JS_RELOAD_TOP);
}

#[test]
fn test_bif_redirect_js_reload_self() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:self :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "js:reload:self");
    assert_eq!(result, REDIR_JS_RELOAD_SELF);
}

#[test]
fn test_bif_redirect_js_reload_self_param() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:self >> some :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "some");
    assert_eq!(result, REDIR_JS_RELOAD_SELF);
}

#[test]
fn test_bif_redirect_negate_fails() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:!redirect; js:reload:top :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>ntsnts</div>");
}

#[test]
fn test_bif_redirect_fails_no_params_1() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:!redirect; :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>ntsnts</div>");
}

#[test]
fn test_bif_redirect_fails_no_params_2() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;__test-nts:}{:!redirect; 301 >> :}{:;__test-nts:}</div>");
    let result = template.render();
    assert_eq!(template.has_error(), true);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, "<div>ntsnts</div>");
}
#[test]
fn test_get_errors() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n :}{:allow: none :}</div>");
    let result = template.render();
    let errors = template.get_error();
    assert_eq!(template.has_error(), true);
    assert_eq!(errors[0], "Error 131 (for) arguments not found  src: {:for; n :}");
    assert_eq!(errors[1], "The delimiter was not found: {:allow: none :}");
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_complete_tpl() {
    let schema = r#"
    {
        "config": {
            "comments": "keep"
        },
        "inherit": {
            "locale": {
                "current": "es",
                "trans": {
                    "es": {
                        "Title": "Título",
                        "ref:greeting-nts": "¡Hola!"
                    }
                }
            }
        }
    }
    "#.trim();
    let out = r#"
<!DOCTYPE html>
<html lang=es>
<head>
    <meta charset=UTF-8>
    <title>Lorem Ipsum Ok nts</title>
</head>
    <body>
        <div class="container">
            <h1>Lorem ipsum Título</h1>
            <div>
                nts
            </div>
                1:nts none
                2:
                    3:hello data nts
                        4:hello data nts
                            5:hello data nts
                                6:hello data nts
                                6
                            5
                        4
                    3
                2
                1:nts none
            <div>
        Lorem:
                Ipsum:
                        Dolor:
                                Sit:
                                        Amet=Consectetur adipiscing elit.
                                        Sed=Do eiusmod tempor incididunt.
                                        Ut=Labore et dolore magna aliqua.
                                        Array:
                                                0=Lorem
                                                1=Ipsum
                                                2=Dolor
                                Enim:
                                        Ad=Minim veniam, quis nostrud exercitation.
                                        Ullamco=Laboris nisi ut aliquip ex ea commodo consequat.
                                        Array:
                                                0=Sed
                                                1=Do
                                                2=Eiusmod
                                                3=Tempor
                                                4=Incididunt
                        Irure:
                                Dolor:
                                        In=Reprehenderit in voluptate.
                                        Excepteur=Sint occaecat cupidatat.
                                        Array:
                                                0=Ut
                                                1=Enim
                                                2=Ad
                Officia:
                        Deserunt:
                                Mollit:
                                        Anim:
                                                Id=Est laborum et dolorum fugiat nulla pariatur.
                                                Sed=Quis nostrud exercitation.
                                Commodo:
                                        Consequat=Duis aute irure dolor in reprehenderit.
                                        Array:
                                                0=Amet
                                                1=Sed
                                                2=Do
                                                3=Eiusmod
                                                4=Tempor
            </div>
            <div>
                is not traversal
                is traversal!
            </div>
            <div>
                    num 0 = 0
                    num 1 = 1
                    num 2 = 2
                    num 3 = 3
                    num 4 = 4
                    num 5 = 5
                    num 6 = 6
                    num 7 = 7
                    num 8 = 8
                    num 9 = 9
            </div>
        </div>
    </body>
</html>
"#.trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("{:include; tests/complete.ntpl :}");
    let result = template.render();
    assert_eq!(template.has_error(), false);
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, out);
}
