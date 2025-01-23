Neutral Web Template System
===========================

Neutral is a **web application template system**, designed to work with **any programming language** via IPC and natively as an IPC or library/crate in Rust.

Safe
----

Neutral is developed in Rust, one of the most secure programming languages. It does not have access to the application's data; it cannot do so because it is designed this way. It implements security mechanisms like "allow," which prevent arbitrary files from being loaded into templates.

Which is not
------------

It is not a programming language, but a markup language, it does not modify data, only its representation.

It is NOT possible:

* Logical operator: varname == varname
* Mathematical operators: varname + varname
* Assignment operators: varname = 1

Localization
------------

Neutral provides powerful and easy-to-use translation utilities... define the translation in a JSON:

    "locale": {
        "current": "en",
        "trans": {
            "en": {
                "Hello": "Hello",
                "ref:greeting-nts": "Hello"
            },
            "es": {
                "Hello": "Hola",
                "ref:greeting-nts": "Hola"
            },
            "de": {
                "Hello": "Hallo",
                "ref:greeting-nts": "Hallo"
            },
            "fr": {
                "Hello": "Bonjour",
                "ref:greeting-nts": "Bonjour"
            },
            "el": {
                "Hello": "Γεια σας",
                "ref:greeting-nts": "Γεια σας"
            }
        }
    }


Now you can use:

    {:trans; Hello :}


Actually you can always use "trans" because if there is no translation it returns the text.

Bif layout (Build-in function)
------------------------------


    .-- open bif
    |    .-- bif name
    |    |   .-- name separator
    |    |   |    .-- params
    |    |   |    |    .-- params/code separatos
    |    |   |    |    |    .-- code
    |    |   |    |    |    |   .-- close bif
    |    |   |    |    |    |   |
    v    v   v    v    v    v   v
    -- ----- - ------- -- ----- --
    {:snippet; varname >>  ...  :}
    ------------------------------
            ^  ----------------
            |         ^
            |         |
            |         `-- source
            `-- Build-in function



Bif example:

    {:filled; varname >>
        Hello!
    :}


Neutral is based on Bifs with block structure, we call the set of nested Bifs of the same level a block:


              .-- {:coalesce;
              |       {:code;
              |           {:code; ... :}
              |           {:code; ... :}
    Block --> |           {:code; ... :}
              |       :}
              |       {:code;
              |           {:code; ... :}
              |       :}
              `-- :}

                  {:coalesce;
              .------ {:code;
              |           {:code; ... :}
    Block --> |           {:code; ... :}
              |           {:code; ... :}
              `------ :}
              .------ {:code;
    Block --> |           {:code; ... :}
              `------ :}
                  :}



Short circuit at block level, if varname is not defined, the following ">>" is not evaluated:

    {:defined; varname >>
        {:code;
            {:code;
                ...
            :}
        :}
    :}


By design all Bifs can be nested and there can be a Bif anywhere in another Bif except in the name.

Data
----

The data is defined in a JSON:

    "data": {
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
    }


And they are displayed with the bif {:; ... :} (var)

Simple variable:

    {:;hello:}


Arrays with the "->" operator

    {:;array->hello:}


Snippets
--------

Snippet is a tool that can be used in a similar way to a function, it defines a snippet:

    {:snippet; name >>
        Any content here, including other snippet.
    :}

From then on you can invoke it like this:

    {:snippet; name :}


Template example
----------------

    {:locale; locale.json :}
    {:include; theme-snippets.ntpl :}
    <!DOCTYPE html>
    <html lang="{:lang;:}">
        <head>
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
    </html>


Native use (Rust)
-----------------

You need two things, a template file and a json schema:

    let schema = json!({
        "config": {
            "comments": "remove"
        },
        "inherit": {
            "locale": {
                "current": "en",
                "trans": {
                    "en": {
                        "Hello nts": "Hello",
                        "ref:greeting-nts": "Hello"
                    },
                    "es": {
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
            "web-site-name": "MySite",
            "varname": "value",
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
        }
    });


Template file.ntpl:

    {:;web-site-name:}


In Rust:

    let template = Template::from_file_value("file.ntpl", schema).unwrap();
    let content = template.render();

    // e.g.: 200
    let status_code = template.get_status_code();

    // e.g.: OK
    let status_text = template.get_status_text();

    // empty if no error
    let status_param = template.get_status_param();

    // act accordingly at this point according to your framework
