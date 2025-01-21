

// Here you can add things but you cannot modify or remove them.
const _SCHEMA: &str =r#"{
    "config": {
        "infinite_loop_max_bifs": 555000,
        "comments": "keep",
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

#[cfg(not(debug_assertions))]
#[test]
// This is only an estimate for minimum quality.
// And possibly this deserves a discussion.
fn test_bif_performance() {
    use std::fs;
    use std::str::FromStr;
    use std::time::Duration;
    use std::time::Instant;

    let mut cpu_frequency: u32 = 0;
    // only in linux
    match fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
        Ok(content) => {
            match u32::from_str(&content.trim()) {
                Ok(freq) => cpu_frequency = freq,
                Err(_error) => assert!(false, "The CPU frequency could not be determined."),
            };
        },
        Err(_error) => assert!(false, "The CPU frequency could not be determined."),
    }

    let target_frequency = 3000000;
    let target_time = 25; // milisegundos
    let estimated_time = (target_frequency * target_time) as f64 / cpu_frequency as f64;

    let mut template = match neutralts::Template::new() {
        Ok(t) => t,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };

    template.merge_schema_str(_SCHEMA).unwrap();
    template.set_src_path("tests/bench.ntpl").unwrap();
    let start = Instant::now();
    template.render();
    let end = Instant::now();
    let duration: Duration = end - start;
    let time = duration.as_millis().to_string().trim().parse::<f64>().unwrap();

    println!("Time: {} millis. estimate max time: {} millis.", duration.as_millis(), estimated_time.trunc());
    assert_eq!(template.has_error(), false);
    assert!(time < estimated_time, "Time must be less.");
}
