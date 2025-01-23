
pub const DEFAULT: &str =r#"{
    "_comment_:License": "License in the terms described in the LICENSE file.",
    "version": "0.0.0",
    "config": {
        "infinite_loop_max_bifs": 555000,
        "_comment_:comments": "keep|remove, default or empty keep",
        "comments": "keep",
        "_comment_:errors": "show|hide, default or empty show",
        "error": {
            "show": true
        },
        "cache_prefix": "domain",
        "cache_dir": "/tmp",
        "working_dir": ""
    },
    "data": {
        "CONTEXT": {
            "CONFIG": {},
            "GET": {},
            "POST": {},
            "SERVER": {},
            "REQUEST": {},
            "FILES": {},
            "COOKIE": {},
            "SESSION": {},
            "ENV": {}
        },
        "__hello-nts": "Hello nts",
        "__ref-hello-nts": "__hello-nts",
        "__test-nts": "nts",
        "__test-arr_nts": [
            "one",
            "two",
            "three"
        ],
        "__test-obj_nts": {
            "level1": "Ok",
            "level1_arr": {
                "level2": "Ok",
                "level2_obj": {
                    "level3": "Ok",
                    "level3_arr": [
                        "one",
                        "two",
                        "three"
                    ]
                }
            }
        }
    },
    "inherit": {
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
        },
        "snippets": {
            "__hello-nts": "<div>{:trans; ref:greeting-nts :}</div>"
        },
        "declare": {
            "any": "*",
            "traversal": "/* \\\\* *\\.\\.*"
        },
        "params": {}
    },
    "__moveto": {},
    "__indir": {},
    "__error": []
}"#;
