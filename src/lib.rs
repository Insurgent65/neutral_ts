#![doc = include_str!("../README.md")]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(static_mut_refs)]

pub mod constants;
mod default_json;
pub mod doc;
pub mod utils;

use crate::constants::*;
use crate::default_json::*;
use crate::utils::*;

use chrono::Utc;
use html_escape::encode_safe;
use md5::{Digest, Md5};
use rand::Rng;
use regex::Regex;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use std::str;
use std::time::Duration;
use std::time::Instant;

//  Build-in function layout
//
//  .-------------------------------------------------------------> open bif
//  | .-----------------------------------------------------------> modifier
//  | |   .-------------------------------------------------------> bif name
//  | |   |   .---------------------------------------------------> bif name separator
//  | |   |   |      .--------------------------------------------> bif params
//  | |   |   |      |       .------------------------------------> params / code separator
//  | |   |   |      |       |             .----------------------> comment
//  | |   |   |      |       |             |   .------------------> bif code
//  | |   |   |      |       |             |   |               .--> close bif
//  | |   |   |      |       |             |   |               |
//  v v   v   v      v       v             |   v               v
//  - - ----- - ------------ -- -----------v------------------ --
//  {:!snippet; snippet_name >> <div>... {:* ... *:} ...</div> :}
//  -------------------------------------------------------------
//          ^  -----------------------------------------------
//          |                      ^
//          |                      |
//          |                      ·------------------------------> bif src
//          ·-----------------------------------------------------> bif: Build-in function

//  Same level Bif:
//
//                  .-----> .-----> {:code:
//                  |       |           {:code: ... :}
//                  |       |           {:code: ... :}
//                  |       |           {:code: ... :}
//  Level block --> |       ·-----> :}
//                  |        -----> {:code: ... :}
//                  |       .-----> {:code:
//                  |       |           {:code: ... :}
//                  ·-----> ·-----> :}

//  Flow
//
//      .-------------------------------.
//      │         new Template          │
//      ·-------------------------------·
//                     |
//                     v
//      .-------------------------------.
//      │       new BlockParser         │ <------.
//      |-------------------------------|        |
//      │      each same level bif      │        |
//      ·-------------------------------·        |
//                     |                         |
//                     v                         |
//      .-------------------------------.        |
//      │           new Bif             │        |
//      |-------------------------------|        |
//      │         nested bifs? ---------│--------·
//      ·-------------------------------·
//                     |
//                     v
//          .----------------------.
//         │       end render       │
//          ·----------------------·

// Inherit, macro: new_child_parse
// -------------------------------
// Inheritance is implemented with this macro. It is also used for the inheritance
// of the application itself.
//
//
// Block level scope example:
//    {:code; <--------------------------.
//        {:* block *:}                  |<---- Block
//        {:param; name >> value :} <----|----- Set "name" for this block and its children
//        {:param; name :} <-------------|----- "name" has the value "value".
//        {:code;                        |
//            {:* child block *:}        |
//            {:param; name :} <---------|----- "name" has the value "value".
//        :}                             |
//    :} <-------------------------------·
//    {:param; name :} <----------------------- outside block, no value or a previous value if any.
//
//
// "include" has a block scope, then:
//    {:code;
//        {:* include for set "snippet-name" *:}
//        {:include; snippet.ntpl :}
//        {:snippet; snippet-name :} {:* Ok, "snippet-name" is set *:}
//    :}
//    {:snippet; snippet-name :} {:* Ko, "snippet-name" is not set *:}
//
// The modifier scope (+) adds the scope to the current level to make it possible to do this:
//    {:+bool; something >>
//        {:include; snippet.ntpl :}
//    :}
//    {:snippet; snippet-name :} {:* Ok, "snippet-name" is set *:}
//
#[macro_use]
mod macros {
    macro_rules! new_child_parse {
        ($self:expr, $source:expr, $scope:expr) => {{
            let mut child_inherit = $self.inherit.clone();
            let shared = &mut $self.shared;

            //  "bif.alias" is used and not "bif.name" because in "var" or "unprintable"
            // its name is an empty string and could have more meanings.
            child_inherit.alias = $self.alias.clone();

            if !$self.file_path.is_empty() {
                child_inherit.current_file = $self.file_path.clone();
            }

            if !$self.dir.is_empty() {
                child_inherit.current_dir = $self.dir.clone();
            }

            // Create a new version of the schema if mod_scope
            // This is necessary because indirections are used,
            // and create_block_schema takes care of that.
            if $scope {
                $self.inherit.create_block_schema(shared);
            }

            let mut block = BlockParser::new(shared, &child_inherit);
            let code = block.parse($source);

            // Update this block with the data generated in the child
            if $scope {
                // el código que estaba aquí lo he movido a la función
                // update_indir para evitar un error de prestamo.
                block.update_indir(&$self.inherit.indir);
            }

            code
        }};
    }
}

struct BifError {
    code: i32,
    msg: String,
    name: String,
    src: String,
}

// Global shared variables
struct Shared {
    schema: Value,
    lang: String,
    comments: String,
    bisf_count: u64,
    bisf_max: u64,
    flags: String,
    exit: bool,
    has_error: bool,
    status_code: String,
    status_text: String,
    status_param: String,
    redirect_js: String,
    // working_dir: String,
}

impl Shared {
    fn new(mut schema: Value) -> Self {
        let bisf_max = schema["config"]["infinite_loop_max_bifs"].as_u64().unwrap();
        let comments = get_from_key(&schema["config"], "comments");
        let lang = get_from_key(&schema["inherit"]["locale"], "current");
        let working_dir = get_from_key(&schema["config"], "working_dir");

        if !working_dir.is_empty() {
            env::set_current_dir(&working_dir).unwrap();
            schema["data"]["CONTEXT"]["working_dir"] = json!(working_dir);
        } else {
            schema["data"]["CONTEXT"]["working_dir"] = json!(env::current_dir().unwrap());
        }

        Shared {
            schema,
            lang,
            comments,
            bisf_count: 0,
            bisf_max,
            flags: String::new(),
            exit: false,
            has_error: false,
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            status_param: String::new(),
            redirect_js: String::new(),
            // working_dir,
        }
    }
}

struct BlockInherit {
    indir: String,
    last_bif_out: bool,
    last_coalesce_out: bool,
    block_count: u64, // u64 is default type in Value nums
    bif_count: u64,   // u64 is default type in Value nums
    alias: String,
    current_file: String,
    current_dir: String,
    include_files: Vec<String>,
    locale_files: Vec<String>,
    data_files: Vec<String>,
}

impl Clone for BlockInherit {
    fn clone(&self) -> Self {
        BlockInherit {
            indir: self.indir.clone(),
            last_bif_out: self.last_bif_out,
            last_coalesce_out: self.last_coalesce_out,
            block_count: self.block_count,
            bif_count: self.bif_count,
            alias: self.alias.clone(),
            current_file: self.current_file.clone(),
            current_dir: self.current_dir.clone(),
            include_files: self.include_files.clone(),
            locale_files: self.locale_files.clone(),
            data_files: self.data_files.clone(),
        }
    }
}

impl BlockInherit {
    fn new() -> Self {
        BlockInherit {
            indir: "block_0".to_string(),
            last_bif_out: false,
            last_coalesce_out: false,
            block_count: 0,
            bif_count: 0,
            alias: String::new(),
            current_file: String::new(),
            current_dir: String::new(),
            include_files: Vec::new(),
            locale_files: Vec::new(),
            data_files: Vec::new(),
        }
    }

    // Create version of data for inheritance at the block level.
    // For performance reasons, instead of inheriting the complete cloned schema,
    // we inherit a reference to the data in the root schema.
    // Therefore, this function should be called before creating data
    // that needs to be inherited to obtain the reference to the storage.
    fn create_block_schema(&mut self, shared: &mut Shared) -> String {
        let prev_id = self.indir.clone();
        let block_id;

        // If this function is called before creating the first block.
        // It may be necessary to initialize values.
        // The first block is not 0, is 1.
        if self.block_count < 1 {
            block_id = "block_1".to_string();
        } else {
            block_id = "block_".to_string() + self.block_count.to_string().as_str();
        }

        // It can be called several times from the same level, in which case
        // it does not need to be cloned again.
        if prev_id != block_id {
            // This line consumes more than 50% of the execution time with an average
            // amount of data, with a lot of data it could consume much more, therefore
            // this site is the first one to look at when improving performance.
            shared.schema["__indir"][&block_id] = shared.schema["__indir"][&prev_id].clone();
        }

        self.indir = block_id.clone();

        block_id
    }
}

pub struct Template<'a> {
    raw: String,
    file_path: &'a str,
    schema: Value,
    shared: Shared,
    time_start: Instant,
    time_elapsed: Duration,
    out: String,
}

/// A struct representing a template that can be rendered.
///
/// This struct is used to handle the rendering of templates.
impl<'a> Template<'a> {
    /// Constructs a new `Template` instance with default settings.
    ///
    /// It allows you to set up a template and schema with different types.
    pub fn new() -> Result<Self, String> {
        let default_schema: Value = match serde_json::from_str(DEFAULT) {
            Ok(value) => value,
            Err(_) => return Err("const DEFAULT is not a valid JSON string".to_string()),
        };
        let shared = Shared::new(default_schema.clone());

        Ok(Template {
            raw: String::new(),
            file_path: "",
            schema: default_schema,
            shared,
            time_start: Instant::now(),
            time_elapsed: Instant::now().elapsed(),
            out: String::new(),
        })
    }

    /// Constructs a new `Template` instance from a file path and a JSON schema.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to the path of the file containing the template content.
    /// * `schema` - A JSON value representing the custom schema to be used with the template.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Template` instance or an error message if:
    /// - The file cannot be read.
    pub fn from_file_value(file_path: &'a str, schema: Value) -> Result<Self, String> {
        let raw: String = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot be read: {}", file_path);
                return Err(e.to_string());
            }
        };
        let mut default_schema: Value = match serde_json::from_str(DEFAULT) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Internal error in const DEFAULT {}, line: {}", file!(), line!());
                return Err("const DEFAULT is not a valid JSON string".to_string());
            }
        };

        merge_schema(&mut default_schema, &schema);
        let shared = Shared::new(default_schema.clone());

        Ok(Template {
            raw,
            file_path,
            schema: default_schema,
            shared,
            time_start: Instant::now(),
            time_elapsed: Instant::now().elapsed(),
            out: String::new(),
        })
    }

    /// Sets the source path of the template.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to the path of the file containing the template content.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if the file cannot be read
    pub fn set_src_path(&mut self, file_path: &'a str) -> Result<(), String> {
        self.file_path = file_path;
        self.raw = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot be read: {}", file_path);
                return Err(e.to_string());
            }
        };

        Ok(())
    }

    /// Sets the content of the template from a string.
    ///
    /// # Arguments
    ///
    /// * `source` - A reference to the new string content to be set as the raw content.
    pub fn set_src_str(&mut self, source: &str) {
        self.raw = source.to_string();
    }

    /// Merges the schema from a file with the current template schema.
    ///
    /// # Arguments
    ///
    /// * `schema_path` - A reference to the path of the file containing the schema content.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if:
    /// - The file cannot be read.
    /// - The file's content is not a valid JSON string.
    pub fn merge_schema_path(&mut self, schema_path: &str) -> Result<(), String> {
        let schema_str: String = match fs::read_to_string(schema_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot be read: {}", schema_path);
                return Err(e.to_string());
            }
        };
        let schema_value: Value = match serde_json::from_str(&schema_str) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Internal error in const DEFAULT {}, line: {}", file!(), line!());
                return Err("const DEFAULT is not a valid JSON string".to_string());
            }
        };
        merge_schema(&mut self.schema, &schema_value);

        Ok(())
    }

    /// Merges the schema from a JSON string with the current template schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - A reference to the JSON string of the schema content.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if:
    /// - The file's content is not a valid JSON string.
    pub fn merge_schema_str(&mut self, schema: &str) -> Result<(), String> {
        let schema_value: Value = match serde_json::from_str(&schema) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Internal error in const DEFAULT {}, line: {}", file!(), line!());
                return Err("const DEFAULT is not a valid JSON string".to_string());
            }
        };
        merge_schema(&mut self.schema, &schema_value);

        Ok(())
    }

    /// Merges the provided JSON value with the current schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - The JSON Value to be merged with the current schema.
    pub fn merge_schema_value(&mut self, schema: Value) {
        merge_schema(&mut self.schema, &schema);
    }

    /// Renders the template content.
    ///
    /// This function initializes the rendering process.
    /// The resulting output is returned as a string.
    ///
    /// # Returns
    ///
    /// The rendered template content as a string.
    pub fn render(&mut self) -> String {
        let inherit = self.init_render();
        self.out = BlockParser::new(&mut self.shared, &inherit).parse(&self.raw);
        self.ends_render();

        self.out.clone()
    }

    // Restore vars for render
    fn init_render(&mut self) -> BlockInherit {
        self.time_start = Instant::now();
        self.shared = Shared::new(self.schema.clone());

        if self.shared.comments.contains("remove") {
            self.raw = remove_comments(&self.raw);
        }

        // init inherit
        let mut inherit = BlockInherit::new();
        let indir = inherit.create_block_schema(&mut self.shared);
        self.shared.schema["__moveto"] = json!({});
        self.shared.schema["__error"] = json!([]);
        self.shared.schema["__indir"] = json!({});
        self.shared.schema["__indir"][&indir] = self.shared.schema["inherit"].clone();
        inherit.current_file = self.file_path.to_string();

        if !self.file_path.is_empty() {
            let path = Path::new(&self.file_path);

            if let Some(parent) = path.parent() {
                inherit.current_dir = parent.display().to_string();
            }
        } else {
            inherit.current_dir = self.shared.schema["data"]["CONTEXT"]["working_dir"].to_string();
        }

        inherit
    }

    // Rendering ends
    fn ends_render(&mut self) {
        self.set_moveto();
        self.replacements();
        self.set_status_code();
        self.time_elapsed = self.time_start.elapsed();
    }

    fn set_status_code(&mut self) {
        let status_code = self.shared.status_code.as_str();

        if status_code >= "400" && status_code <= "599" {
            self.out = format!("{} {}", self.shared.status_code, self.shared.status_text);

            return;
        }

        if status_code == "301"
            || status_code == "302"
            || status_code == "303"
            || status_code == "307"
            || status_code == "308"
        {
            self.out = format!(
                "{} {}\n{}",
                self.shared.status_code, self.shared.status_text, self.shared.status_param
            );

            return;
        }

        if !self.shared.redirect_js.is_empty() {
            self.out = self.shared.redirect_js.clone();

            return;
        }
    }

    fn set_moveto(&mut self) {
        if let Value::Object(data_map) = &self.shared.schema["__moveto"] {
            for (_key, value) in data_map {
                if let Value::Object(inner_map) = value {
                    for (inner_key, inner_value) in inner_map {
                        let mut tag;

                        // although it should be "<tag" or "</tag" it also supports
                        // "tag", "/tag", "<tag>" and "</tag>
                        if !inner_key.starts_with("<") {
                            tag = format!("<{}", inner_key);
                        } else {
                            tag = inner_key.to_string();
                        }
                        if tag.ends_with(">") {
                            tag = tag[..tag.len() - 1].to_string();
                        }

                        // if it does not find it, it does nothing
                        let position = find_tag_position(&self.out, &tag);
                        if let Some(pos) = position {
                            let mut insert = inner_value.as_str().unwrap().to_string();
                            insert = format!("{}", &insert);
                            self.out.insert_str(pos, &insert);
                        }
                    }
                }
            }
        }
    }

    fn replacements(&mut self) {
        let pattern = format!(r"\s*{}", BACKSPACE);
        let re = Regex::new(&pattern).expect("Failed to create regex with constant pattern");
        self.out = re.replace_all(&self.out, "").to_string();

        // UNPRINTABLE should be substituted after BACKSPACE
        self.out = self.out.replace(UNPRINTABLE, "");
    }

    /// Retrieves the status code.
    ///
    /// The status code is "200" unless "exit", "redirect" is used or the
    /// template contains a syntax error, which will return a status code
    /// of "500". Although the codes are numeric, a string is returned.
    ///
    /// # Returns
    ///
    /// A reference to the status code as a string.
    pub fn get_status_code(&self) -> &String {
        &self.shared.status_code
    }

    /// Retrieves the status text.
    ///
    /// It will correspond to the one set by the HTTP protocol.
    ///
    /// # Returns
    ///
    /// A reference to the status text as a string.
    pub fn get_status_text(&self) -> &String {
        &self.shared.status_text
    }

    /// Retrieves the status parameter.
    ///
    /// Some statuses such as 301 (redirect) may contain additional data, such
    /// as the destination URL, and in similar cases “param” will contain
    /// that value.
    ///
    /// # Returns
    ///
    /// A reference to the status parameter as a string.
    pub fn get_status_param(&self) -> &String {
        &self.shared.status_param
    }

    /// Checks if there is an error.
    ///
    /// If any error has occurred, in the parse or otherwise, it will return true.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether there is an error.
    pub fn has_error(&self) -> bool {
        self.shared.has_error
    }

    /// Get bifs errors list
    ///
    /// # Returns
    ///
    /// * `Value`: A clone of the value with the list of errors in the bifs during rendering.
    pub fn get_error(&self) -> Value {
        self.shared.schema["__error"].clone()
    }

    /// Retrieves the time duration for template rendering.
    ///
    /// # Returns
    ///
    /// The time duration elapsed .
    pub fn get_time_duration(&self) -> Duration {
        let duration: std::time::Duration = self.time_elapsed;

        duration.clone()
    }
}

struct BlockParser<'a> {
    shared: &'a mut Shared,
    inherit: BlockInherit,
    _none: &'a str,
}

impl Drop for BlockParser<'_> {
    fn drop(&mut self) {
        // release memory
        let block_id = "block_".to_string() + self.inherit.block_count.to_string().as_str();
        if block_id == self.inherit.indir {
            if is_defined_key(&self.shared.schema["__indir"], &block_id) {
                self.shared.schema["__indir"][&block_id] = json!({});
            }
        }
    }
}

impl<'a> BlockParser<'a> {
    fn new(shared: &'a mut Shared, inherit: &BlockInherit) -> Self {
        let mut inherit = inherit.clone();
        inherit.block_count += 1;

        BlockParser {
            shared,
            inherit,
            _none: "",
        }
    }

    fn update_indir(&mut self, indir: &String) {
        self.shared.schema["__indir"][indir] =
            self.shared.schema["__indir"][&self.inherit.indir].clone();
    }

    fn parse(&mut self, raw_source: &'a str) -> String {
        let blocks;

        match extract_blocks(raw_source) {
            Ok(b) => {
                blocks = b;
            }
            Err(p) => {
                self.shared.status_code = "500".to_string();
                self.shared.status_param = format!("Unmatched block at position {}", p);
                eprintln!("Unmatched block at position {}", p);

                if let Some(text) = STATUS_CODES.get(self.shared.status_code.as_str()) {
                    self.shared.status_text = text.to_string();
                } else {
                    self.shared.status_text = EMPTY_STRING;
                }

                return EMPTY_STRING;
            }
        }

        let mut prev_end = 0;
        let mut out = String::new();
        for (start, end) in blocks {
            let is_comment = raw_source[start..end].starts_with(BIF_COMMENT_OPEN);
            let is_short_circuit_coalesce =
                self.inherit.last_coalesce_out && self.inherit.alias == "coalesce";

            if self.shared.exit {
                return out.clone();
            }

            if prev_end < start {
                out += &raw_source[prev_end..start];
            }

            if !is_comment && !is_short_circuit_coalesce {
                let mut bif =
                    Bif::new(&raw_source[start..end], &mut self.shared, &mut self.inherit);
                out += &bif.parse();
            }

            prev_end = end;
        }
        out += &raw_source[prev_end..];

        out.trim().to_string()
    }
}

struct Bif<'a> {
    raw: &'a str,
    shared: &'a mut Shared,
    inherit: &'a mut BlockInherit,
    src: String,
    name: String,
    alias: String,
    code: String,
    params: String,
    flags: String,
    mod_filter: bool,
    mod_negate: bool,
    mod_upline: bool,
    mod_scope: bool,
    file_path: String,
    dir: String,
    out: String,
    _none: &'a str,
}

impl<'a> Bif<'a> {
    fn new(raw_source: &'a str, shared: &'a mut Shared, inherit: &'a mut BlockInherit) -> Self {
        shared.bisf_count += 1;
        let count = shared.bisf_count;
        inherit.bif_count = shared.bisf_count;

        if count > shared.bisf_max {
            panic!(
                "Infinite loop? {} bifs of {} max have been created.",
                shared.bisf_max, count
            );
        }

        Bif {
            raw: raw_source, // should not be modified
            shared,
            inherit,
            src: String::new(),
            name: String::new(),
            alias: String::new(),
            code: String::new(),
            params: String::new(),
            flags: String::new(),
            mod_filter: false,
            mod_negate: false,
            mod_upline: false,
            mod_scope: false,
            file_path: String::new(),
            dir: String::new(),
            out: String::new(),
            _none: "",
        }
    }

    // Divides the bif into its parts and executes the bif parse function.
    fn parse(&mut self) -> String {
        let bif = strip_prefix_suffix(&self.raw, BIF_OPEN, BIF_CLOSE);
        let result;

        if let Some((name, src)) = bif.split_once(BIF_NAME) {
            self.name = name.to_string();
            if name == "neutral" {
                self.src = src.to_string();
            } else {
                self.src = src.trim().to_string();
            }
        } else {
            let show_error = self.shared.schema["config"]["error"]["show"].as_bool().unwrap();
            let error_line = format!("The delimiter was not found: {}", self.raw);
            let error_line = error_line.replace(|c: char| c == '\n' || c == '\r', " ");

            if let Some(Value::Array(ref mut errors)) = self.shared.schema.get_mut("__error") {
                errors.push(json!(error_line));
            }

            if show_error {
                eprintln!("{}", error_line);
            }

            self.shared.has_error = true;

            return EMPTY_STRING;
        }
        self.name = self.set_modifiers();
        self.alias = self.name.clone();

        // exec the function of each bif
        match &self.name[..] {
            "" => result = self.parse_bif_var(),
            "allow" => result = self.parse_bif_allow(),
            "array" => result = self.parse_bif_array(),
            "bool" => result = self.parse_bif_bool(),
            "coalesce" => result = self.parse_bif_coalesce(),
            "code" => result = self.parse_bif_code(),
            "count" => result = self.parse_bif_count(),
            "data" => result = self.parse_bif_data(),
            "date" => result = self.parse_bif_date(),
            "declare" => result = self.parse_bif_declare(),
            "defined" => result = self.parse_bif_defined(),
            "each" => result = self.parse_bif_each(),
            "else" => result = self.parse_bif_else(),
            "eval" => result = self.parse_bif_eval(),
            "exit" => result = self.parse_bif_exit(),
            "filled" => result = self.parse_bif_filled(),
            "flg" => result = self.parse_bif_flg(),
            "for" => result = self.parse_bif_for(),
            "hash" => result = self.parse_bif_hash(),
            "include" => result = self.parse_bif_include(),
            "lang" => result = self.parse_bif_lang(),
            "locale" => result = self.parse_bif_locale(),
            "moveto" => result = self.parse_bif_moveto(),
            "neutral" => result = self.parse_bif_neutral(),
            "param" => result = self.parse_bif_param(),
            "rand" => result = self.parse_bif_rand(),
            "redirect" => result = self.parse_bif_redirect(),
            "replace" => result = self.parse_bif_replace(),
            "snippet" => result = self.parse_bif_snippet(),
            "trans" => result = self.parse_bif_trans(),
            _ => result = self.parse_bif_unknown(),
        }

        match result {
            Ok(()) => (),
            Err(e) => {
                let show_error = self.shared.schema["config"]["error"]["show"].as_bool().unwrap();
                let error_line = format!("Error {} ({}) {}  src: {}", e.code, e.name, e.msg, e.src);
                let error_line = error_line.replace(|c: char| c == '\n' || c == '\r', " ");

                if let Some(Value::Array(ref mut errors)) = self.shared.schema.get_mut("__error") {
                    errors.push(json!(error_line));
                }

                if show_error {
                    eprintln!("{}", error_line);
                }

                self.shared.has_error = true;
            }
        }

        self.inherit.last_bif_out = !self.out.is_empty();
        self.inherit.last_coalesce_out = self.inherit.last_bif_out;

        if self.mod_upline {
            self.out = BACKSPACE.to_string() + &self.out;
            return self.out.trim().to_string();
        } else {
            return self.out.trim().to_string();
        }
    }

    //  Determines which modifiers are being used
    //
    //    .------ modifier
    //    |
    //    v
    //  {:!snippet; ...
    //
    fn set_modifiers(&mut self) -> String {
        let mut index = 0;
        while index < self.name.len() {
            let start = &self.name[index..index + 1];
            if start == BIF_MOD_FILTER
                || start == BIF_MOD_NEGATE
                || start == BIF_MOD_UPLINE
                || start == BIF_MOD_SCOPE
            {
                match start {
                    BIF_MOD_FILTER => self.mod_filter = true,
                    BIF_MOD_NEGATE => self.mod_negate = true,
                    BIF_MOD_UPLINE => self.mod_upline = true,
                    BIF_MOD_SCOPE => self.mod_scope = true,
                    _ => unreachable!(),
                }
                index += 1;
            } else {
                break;
            }
        }

        self.name[index..].to_string()
    }

    // Get key from schema data o local data
    //
    // {
    //     "config": {},
    //     "inherit": {},
    //     "data": {}  <------------ schema data get from
    //     "__indir": {
    //          "X": {
    //             "data": {} <----- local data get from
    //     ...
    // }
    fn get_data(&self, name: &str) -> String {
        if name.starts_with("local::") {
            let local_name = name.strip_prefix("local::").unwrap_or(name);
            get_from_key(
                &self.shared.schema["__indir"][&self.inherit.indir]["data"],
                local_name,
            )
        } else {
            get_from_key(&self.shared.schema["data"], name)
        }
    }

    // Set key to schema data
    //
    // {
    //     "config": {},
    //     "inherit": {},
    //     "data": {}  <-------- set to
    // }
    fn set_data(&mut self, name: &str, value: &str) {
        self.shared.schema["data"][name] = json!(value);
    }

    // Get key from schema locale, an indirection is used instead of its initial position
    // {
    //     "config": {},
    //     "inherit": {
    //     "locale": { ------------------.
    //        "current": "en",           |
    //        "trans": {                 |
    //           "es": {}                |
    //         }                         | moved on init Template
    //     },                            |
    //     "data": {},                   |
    //     "__indir": {                  |
    //          "X": {                   |
    //             "locale": { <---------·
    //                 "trans": {
    //                     "es": {} <----- get from
    //     ...
    // }
    fn get_trans(&self, text: &str) -> String {
        get_from_key(
            &self.shared.schema["__indir"][&self.inherit.indir]["locale"]["trans"]
                [&self.shared.lang],
            text,
        )
    }

    /*
        dynamic evaluation

        This is not allowed: {:;{:;refvarname:}:}
        Use instead: {:; {:allow; allowed >> {:;refvarname:} :} :}
    */
    fn contains_allow(&self, source: &str) -> bool {
        for allow in BIF_ALLOWED {
            if source.contains(allow) {
                return true;
            }
        }

        let source = &remove_comments(&source);
        if source.starts_with(BIF_OPEN) && source.ends_with(BIF_CLOSE) {
            return false;
        } else {
            return true;
        }
    }

    // Split params/code and parse params if parse is true.
    // It is possible that it has no parameters, in which case
    // it is all code and the parameters are an empty string.
    // To set flags, parameters are required.
    //
    //                   .------------------------------> params
    //                   |       .----------------------> separator
    //                   |       |
    //                   |       |                 .----> code
    //                   |       |                 |
    //                   v       v                 v
    //              ------------ -- ------------------------------
    //  {:!snippet; snippet_name >> <div>... {:* ... *:} ...</div> :}
    fn extract_params_code(&mut self, parse: bool) -> bool {
        let position = get_code_position(&self.src);
        let has_code: bool = position.is_some();

        if has_code {
            let code_pos = position.unwrap();
            self.params = self.src[0..code_pos].trim().to_string();
            self.code = self.src[code_pos + BIF_CODE.len()..].trim().to_string();
        } else {
            self.params = EMPTY_STRING;
            self.code = self.src.trim().to_string();
        }

        if parse {
            if self.params.contains(BIF_OPEN) {
                self.shared.flags = EMPTY_STRING;
                self.params = new_child_parse!(self, &self.params, false);
                self.flags = self.shared.flags.clone();
            }
        }

        has_code
    }

    /*
        unknown bif
    */
    fn parse_bif_unknown(&mut self) -> Result<(), BifError> {
        self.alias = "unknown".to_string();

        Err(BifError {
            code: 101,
            msg: "unknown bif".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })
    }

    /*
        {:;varname:}
        {:;:}
    */
    fn parse_bif_var(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 102,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        // Unprintable: {:;:} / {:; :}
        if self.src.is_empty() {
            // "bif.alias" is used and not "bif.name" because in "var" or "unprintable"
            // its name is an empty string.
            self.alias = "unprintable".to_string();

            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        // Var: {:;varname:}
        self.alias = "var".to_string();

        // For security requires {:allow; in some cases.
        if self.src.contains(BIF_OPEN) {
            if !self.contains_allow(&self.src) {
                self.out = EMPTY_STRING;

                return Err(BifError {
                    code: 103,
                    msg: "insecure varname".to_string(),
                    name: self.alias.clone(),
                    src: self.src.clone(),
                });
            }

            let var_name = &new_child_parse!(self, &self.src, self.mod_scope);
            self.out = self.get_data(var_name).to_string();

            return Ok(());
        }

        let var_name = &self.src;
        self.out = self.get_data(var_name).to_string();

        return Ok(());
    }

    /*
        {:allow; {:flg; partial casein replace :} name >> ... :}
    */
    fn parse_bif_allow(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                code: 104,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        let mut found = String::new();
        let words_string = get_from_key(
            &self.shared.schema["__indir"][&self.inherit.indir]["declare"],
            &self.params,
        );

        if words_string.is_empty() {
            return Err(BifError {
                code: 105,
                msg: self.params.clone() + " declared is empty",
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let mut words_list: Vec<&str> = words_string.split_whitespace().collect();
        self.code = new_child_parse!(self, &self.code, self.mod_scope);

        for word in &mut words_list {
            let lower_haystack;
            let mut haystack = &self.code;
            let mut pattern = word.to_string().clone();

            if self.flags.contains("|partial|") || self.flags.contains("|replace|") {
                pattern = format!("{}{}{}", "*", pattern, "*");
            }

            if self.flags.contains("|casein|") {
                pattern = pattern.to_lowercase();
                lower_haystack = self.code.clone().to_lowercase();
                haystack = &lower_haystack;
            }

            if wildcard_match(haystack, &pattern) {
                found = word.to_string();
                break;
            }
        }

        if !found.is_empty() ^ self.mod_negate {
            if self.flags.contains("|replace|") {
                found = found.replace("~", "");
                found = found.replace("*", "");
                found = found.replace("?", "");
                found = found.replace(".", "");
                self.out = found.to_string();
            } else {
                self.out = self.code.to_string();
            }
        } else {
            self.out = EMPTY_STRING;
        }

        Ok(())
    }

    /*
        {:array; varname >> ... :}
    */
    fn parse_bif_array(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                code: 106,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if is_array_key(&self.shared.schema["data"], &self.params) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }
            self.out = self.code.to_string();
        }

        Ok(())
    }

    /*
        {:bool; varname >> ... :}
    */
    fn parse_bif_bool(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                code: 107,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if is_bool_key(&self.shared.schema["data"], &self.params) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }
            self.out = self.code.to_string();
        }

        Ok(())
    }

    /*
       {:coalesce;
           {:code;  :}
           {:code; this is output :}
           {:code; ... :}
       :}
    */
    fn parse_bif_coalesce(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(BifError {
                code: 108,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        // This var so as not to overwrite the original: inherit.last_bif_out
        self.inherit.last_coalesce_out = false;
        self.out = new_child_parse!(self, &self.src, self.mod_scope);

        Ok(())
    }

    /*
        {:code; ...  :}
        {:code; {:flags; safe noparse encode_tags encode_tags_after encode_bifs :} >>  <div>...</div>  :}
    */
    fn parse_bif_code(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(BifError {
                code: 109,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if self.flags.contains("|safe|") {
            self.code = encode_safe(&self.code).to_string();
            self.code = self.code.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
            self.code = self.code.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
        } else {
            if self.flags.contains("|encode_tags|") {
                self.code = encode_safe(&self.code).to_string();
            }

            if self.flags.contains("|encode_bifs|") {
                self.code = self.code.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
                self.code = self.code.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
            }

            if !self.flags.contains("|noparse|") {
                if self.code.contains(BIF_OPEN) {
                    self.code = new_child_parse!(self, &self.code, self.mod_scope);
                }
            }
        }

        if self.flags.contains("|encode_tags_after|") {
            self.code = encode_safe(&self.code).to_string();
        }

        self.out = self.code.to_string();

        Ok(())
    }

    /*
        {:count; name >> 0 :}
        {:count; name :}
    */
    fn parse_bif_count(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 110,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let is_set = self.extract_params_code(true);

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        if is_set {
            let count_name = self.params.clone();
            let count_value = match self.code.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(BifError {
                        code: 111,
                        msg: "argument is not a number".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    });
                }
            };

            self.set_data(&count_name, &count_value.to_string());
            self.out = EMPTY_STRING;
        } else {
            let count_name = self.code.clone();
            let count_value = match self.get_data(&count_name).parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(BifError {
                        code: 112,
                        msg: "argument is not a number".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    });
                }
            };
            let new_value = count_value + 1;

            self.set_data(&count_name, &new_value.to_string());
            self.out = count_value.to_string();
        }

        Ok(())
    }

    /*
        {:data; file-path :} {:* local data *}
    */
    fn parse_bif_data(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                code: 113,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        self.file_path = self.code.clone();

        // For security requires {:allow;
        if self.file_path.contains(BIF_OPEN) {
            if !self.contains_allow(&self.file_path) {
                return Err(BifError {
                    code: 114,
                    msg: "insecure file name".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }
            self.file_path = new_child_parse!(self, &self.code, false);
        }

        if self.file_path.starts_with("#") {
            self.file_path.remove(0);
            self.file_path = format!("{}{}", self.inherit.current_dir, self.file_path);
        }

        let path = Path::new(&self.file_path);
        if !Path::new(path).exists() {
            if self.flags.contains("|require|") {
                return Err(BifError {
                    code: 115,
                    msg: "file not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            } else {
                self.out = EMPTY_STRING;

                return Ok(());
            }
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();

        if self.mod_negate {
            if self.inherit.data_files.contains(&canonical_path) {
                self.out = EMPTY_STRING;

                return Ok(());
            }
        }

        self.inherit.data_files.push(canonical_path);
        let mut file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());

        if !self.flags.contains("|noparse|") {
            // Parse possible bifs included in json
            if file_raw.contains(BIF_OPEN) {
                file_raw = new_child_parse!(self, &file_raw, false);
            }
        }

        let data: Value = match serde_json::from_str(&file_raw) {
            Ok(value) => value,
            Err(_) => {
                return Err(BifError {
                    code: 116,
                    msg: "not a valid JSON file".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let indir = &self.inherit.create_block_schema(&mut self.shared);

        // Merge new locale data in curren local data.
        merge_schema(
            &mut self.shared.schema["__indir"][indir]["data"],
            &data["data"],
        );

        self.out = UNPRINTABLE.to_string();

        Ok(())
    }

    /*
        {:date;  :} timestamp
        {:date; %Y-%m-%d %H:%M:%S  :} UTC
    */
    fn parse_bif_date(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 117,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        let now = Utc::now();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.src.trim().to_string();
        }

        if self.code.is_empty() {
            self.out = now.timestamp().to_string();
        } else {
            self.out = now.format(&self.src).to_string();
        }

        Ok(())
    }

    /*
        {:declare; name >> words list :}
    */
    fn parse_bif_declare(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 118,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if self.inherit.current_file.contains(SNIPPETS_FILES) {
            self.inherit.create_block_schema(&mut self.shared);
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
                self.code = self.code.replace(UNPRINTABLE, "");
            }
            self.shared.schema["__indir"][&self.inherit.indir]["declare"][&self.params] =
                json!(&self.code);

            self.out = EMPTY_STRING;
        } else {
            return Err(BifError {
                code: 119,
                msg: "declare cannot be set here".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        Ok(())
    }

    /*
        {:defined; varname >> ... :}
    */
    fn parse_bif_defined(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                code: 120,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if is_defined_key(&self.shared.schema["data"], &self.params) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }
            self.out = self.code.to_string();
        }

        Ok(())
    }

    /*
        {:each; array-name name-for-key name-for-value  >>
            {:;name-for-key:}={:;name-for-value:}
        :}
    */
    fn parse_bif_each(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 121,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        let mut parts = self.params.split_whitespace();

        let array_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    code: 122,
                    msg: "arguments not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let key_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    code: 123,
                    msg: "arguments 'key' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let val_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    code: 124,
                    msg: "arguments 'value' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let tmp: String = format!("{}{}", "/", array_name);
        let mut array = tmp.replace(BIF_ARRAY, "/");
        let restore_key = self.get_data(&key_name);
        let restore_val = self.get_data(&val_name);

        let data_storage;
        if array.starts_with("/local::") {
            array = array.replace("/local::", "/");
            data_storage = &self.shared.schema["__indir"][&self.inherit.indir]["data"];
        } else {
            data_storage = &self.shared.schema["data"];
        }

        if let Some(data_value) = data_storage.pointer(&array) {
            match data_value.to_owned() {
                Value::Object(obj) => {
                    for (key, val) in obj.iter() {
                        self.parse_bif_each_iter(&key_name, &val_name, key, val);
                    }
                }
                Value::Array(arr) => {
                    for (key, val) in arr.iter().enumerate() {
                        self.parse_bif_each_iter(&key_name, &val_name, &key.to_string(), val);
                    }
                }
                _ => {}
            }
        }

        self.set_data(&restore_key, &restore_key);
        self.set_data(&restore_val, &val_name);

        Ok(())
    }

    fn parse_bif_each_iter(&mut self, key_name: &str, val_name: &str, key: &String, val: &Value) {
        self.shared.schema["data"][key_name] = json!(key);
        self.shared.schema["data"][val_name] = json!(val);
        self.out += &new_child_parse!(self, &self.code, false);
    }

    /*
       {:else; ... :}
       {:code; :}{:else; this is output :}
       {:code; not empty :}{:!else; this is output :}
    */
    fn parse_bif_else(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                code: 125,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if self.inherit.last_bif_out ^ self.mod_negate {
            self.out = EMPTY_STRING;

            return Ok(());
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.out = self.code.to_string();

        Ok(())
    }

    /*
        {:eval; code >> ... {:;__eval__:} ... :} {:* embbedding *:}
    */
    fn parse_bif_eval(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                code: 126,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(false);

        if self.params.contains(BIF_OPEN) {
            self.params = new_child_parse!(self, &self.params, self.mod_scope);
        }

        if (self.params != EMPTY_STRING) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                let restore_eval = self.get_data("__eval__");
                self.set_data("__eval__", &self.params.clone());
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
                self.set_data("__eval__", &restore_eval);
            }
            self.out = self.code.clone();
        } else {
            self.out = EMPTY_STRING;
        }

        Ok(())
    }

    /*
        {:exit; :}
        {:exit; 404 :}
        {:!exit; 202 :} {:* only sets the status code :}
        {:exit; 301 >> /page :}
    */
    fn parse_bif_exit(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                code: 127,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let has_status_params = self.extract_params_code(true);

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.src, false);
        }

        let mut status_code = "200";
        let mut status_param = "";

        if has_status_params {
            if !self.params.is_empty() {
                status_code = self.params.as_str();
            }
            status_param = &self.code;
        } else {
            if !self.code.is_empty() {
                status_code = self.code.as_str();
            }
        }

        self.shared.status_code = status_code.to_string();
        self.shared.status_param = status_param.to_string();

        if let Some(text) = STATUS_CODES.get(status_code) {
            self.shared.status_text = text.to_string();
        } else {
            self.shared.status_text = EMPTY_STRING;
        }

        self.shared.exit = true ^ self.mod_negate;
        self.out = EMPTY_STRING;

        Ok(())
    }

    /*
        {:filled; varname >> ... :}

        *** It is only not filled if it has nothing "", if it is not defined
        or is null, the rest “false”, “0” etc. is something.
    */
    fn parse_bif_filled(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                code: 128,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if !is_empty_key(&self.shared.schema["data"], &self.params) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }

            self.out = self.code.to_string();
        }

        Ok(())
    }

    /*
        {:flg; flag-name1 flag-name2 ... :}
        {:code; {:flg; safe :} >>  <div>...</div> :}
    */
    fn parse_bif_flg(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_upline || self.mod_scope {
            return Err(BifError {
                code: 129,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, false);
        }

        let flags = format!(" {} ", self.code);
        self.shared.flags = flags.replace(" ", "|");
        self.out = EMPTY_STRING;

        Ok(())
    }

    /*
       {:for; varname 1 10 >>
           var is:{:;varname:}
       :}
    */
    fn parse_bif_for(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 130,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        self.params = self.params.replace("..", " ");
        let mut parts = self.params.split_whitespace();

        let var_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    code: 131,
                    msg: "arguments not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let from = match parts.next() {
            Some(value) => match value.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(BifError {
                        code: 132,
                        msg: "argument is not a number".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            },
            None => {
                return Err(BifError {
                    code: 133,
                    msg: "arguments 'from' and 'to' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let to = match parts.next() {
            Some(value) => match value.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(BifError {
                        code: 134,
                        msg: "argument is not a number".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            },
            None => {
                return Err(BifError {
                    code: 135,
                    msg: "arguments 'to' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let range = if from > to {
            (to..=from).rev().collect::<Vec<i32>>()
        } else {
            (from..=to).collect::<Vec<i32>>()
        };

        let restore_var = self.get_data(&var_name);
        for i in range {
            self.set_data(&var_name, &i.to_string());
            self.out += &new_child_parse!(self, &self.code, self.mod_scope);
        }
        self.set_data(&var_name, &restore_var);

        Ok(())
    }

    /*
        {:hash;  :}
        {:hash; text :}
    */
    fn parse_bif_hash(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 136,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.code = self.src.trim().to_string();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.code.trim().to_string();
        }

        if self.code.is_empty() {
            let mut hasher = Md5::new();
            let mut rng = rand::thread_rng();
            let rand = rng.gen_range(100000000..=999999999).to_string();
            hasher.update(&rand);
            self.out = format!("{:x}", hasher.finalize())
        } else {
            let mut hasher = Md5::new();
            hasher.update(&self.code);
            self.out = format!("{:x}", hasher.finalize());
        }

        Ok(())
    }

    /*
        {:include; file-path :}
        {:include; {:flg; require safe noparse :} >> file-path :}
    */
    fn parse_bif_include(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                code: 137,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        self.file_path = self.code.clone();

        // For security requires {:allow;
        if self.file_path.contains(BIF_OPEN) {
            if !self.contains_allow(&self.file_path) {
                return Err(BifError {
                    code: 138,
                    msg: "insecure file name".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }
            self.file_path = new_child_parse!(self, &self.code, false);
        }

        if self.file_path.starts_with("#") {
            self.file_path.remove(0);
            self.file_path = format!("{}{}", self.inherit.current_dir, self.file_path);
        }

        let path = Path::new(&self.file_path);
        if !path.exists() {
            if self.flags.contains("|require|") {
                return Err(BifError {
                    code: 139,
                    msg: "file not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            } else {
                return Ok(());
            }
        }

        if let Some(parent) = path.parent() {
            self.dir = parent.display().to_string();
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();

        if self.mod_negate {
            if self.inherit.include_files.contains(&canonical_path) {
                self.out = EMPTY_STRING;

                return Ok(());
            }
        }

        if self.flags.contains("|safe|") {
            self.code = fs::read_to_string(&self.file_path).unwrap_or("".to_string());
            self.code = encode_safe(&self.code).to_string();
            self.code = self.code.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
            self.code = self.code.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
            self.out = self.code.clone();

            return Ok(());
        }

        if self.flags.contains("|noparse|") {
            self.code = fs::read_to_string(&self.file_path).unwrap_or("".to_string());
            self.out = self.code.clone();

            return Ok(());
        }

        self.inherit.include_files.push(canonical_path);

        let mut file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());
        if self.shared.comments.contains("remove") {
            file_raw = remove_comments(&file_raw);
        }

        self.out = new_child_parse!(self, &file_raw, true);

        Ok(())
    }

    /*
       {:lang; ... :}
    */
    fn parse_bif_lang(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 140,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.out = self.shared.lang.to_string();

        Ok(())
    }

    /*
        {:locale; file-path :}
    */
    fn parse_bif_locale(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                code: 141,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if self.flags.contains("|inline|") {
            // Parse possible bifs included in json
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
            }

            let locale: Value = match serde_json::from_str(&self.code) {
                Ok(value) => value,
                Err(_) => {
                    return Err(BifError {
                        code: 142,
                        msg: "not a valid JSON string".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            };

            let indir = &self.inherit.create_block_schema(&mut self.shared);

            // Merge new locale data in curren locale.
            merge_schema(&mut self.shared.schema["__indir"][indir]["locale"], &locale);

            self.out = EMPTY_STRING;

            return Ok(());
        }

        self.file_path = self.code.clone();

        // For security requires {:allow;
        if self.file_path.contains(BIF_OPEN) {
            if !self.contains_allow(&self.file_path) {
                return Err(BifError {
                    code: 143,
                    msg: "insecure file name".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }
            self.file_path = new_child_parse!(self, &self.code, false);
        }

        if self.file_path.starts_with("#") {
            self.file_path.remove(0);
            self.file_path = format!("{}{}", self.inherit.current_dir, self.file_path);
        }

        let path = Path::new(&self.file_path);
        if !Path::new(path).exists() {
            if self.flags.contains("|require|") {
                return Err(BifError {
                    code: 144,
                    msg: "file not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            } else {
                return Ok(());
            }
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if self.mod_negate {
            if self.inherit.locale_files.contains(&canonical_path) {
                self.out = EMPTY_STRING;

                return Ok(());
            }
        }

        self.inherit.locale_files.push(canonical_path);
        let mut file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());

        if !self.flags.contains("|noparse|") {
            // Parse possible bifs included in json
            if file_raw.contains(BIF_OPEN) {
                file_raw = new_child_parse!(self, &file_raw, false);
            }
        }

        let locale: Value = match serde_json::from_str(&file_raw) {
            Ok(value) => value,
            Err(_) => {
                return Err(BifError {
                    code: 145,
                    msg: "not a valid JSON file".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let indir = &self.inherit.create_block_schema(&mut self.shared);

        // Merge new locale data in curren locale.
        merge_schema(&mut self.shared.schema["__indir"][indir]["locale"], &locale);

        self.out = UNPRINTABLE.to_string();

        Ok(())
    }

    /*
        {:moveto; <tag >> ... :}
        {:moveto; </tag >> ... :}
    */
    fn parse_bif_moveto(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 146,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        let mut moveto = json!({});
        let mut hasher = Md5::new();

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        // the same code moves only once
        hasher.update(&self.code);
        let code_hash = hasher.finalize();
        let code_hash = format!("{:x}", code_hash);

        moveto[&self.params] = json!(&self.code);
        self.shared.schema["__moveto"][&code_hash] = moveto;
        self.out = EMPTY_STRING;

        Ok(())
    }

    /*
        {:neutral; ... :}
    */
    fn parse_bif_neutral(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 147,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.out = format!(
            "{}{}{}{}{}",
            BIF_OPEN, self.name, BIF_NAME, self.src, BIF_CLOSE
        );

        Ok(())
    }

    /*
        Play param: {:param; param-name :}
        Set param:  {:param; param-name >> content to set :}
    */
    fn parse_bif_param(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 148,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let is_set = self.extract_params_code(true);
        if is_set {
            if self.inherit.alias == "code" {
                if self.code.contains(BIF_OPEN) {
                    self.code = new_child_parse!(self, &self.code, self.mod_scope);
                }

                self.inherit.create_block_schema(&mut self.shared);
                self.shared.schema["__indir"][&self.inherit.indir]["params"][&self.params] =
                    json!(&self.code);
                self.out = EMPTY_STRING;

                return Ok(());
            } else {
                return Err(BifError {
                    code: 149,
                    msg: "param cannot be set here".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }
        } else {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }

            self.code = get_from_key(
                &self.shared.schema["__indir"][&self.inherit.indir]["params"],
                &self.code,
            );
            self.out = self.code.to_string();

            return Ok(());
        }
    }

    /*
        {:rand;  :}
        {:rand; 1..100 :}
    */
    fn parse_bif_rand(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 150,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let mut rng = rand::thread_rng();
        self.code = self.src.trim().to_string();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.src.trim().to_string();
        }

        if self.code.is_empty() {
            self.out = rng.gen_range(100000000..=999999999).to_string();
        } else {
            // TODO comprobar rangos
            self.code = self.code.replace("..", " ");
            let mut parts = self.code.split_whitespace();

            let from = match parts.next() {
                Some(value) => match value.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(BifError {
                            code: 151,
                            msg: "argument is not a number".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        })
                    }
                },
                None => {
                    return Err(BifError {
                        code: 152,
                        msg: "arguments not found".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            };

            let to = match parts.next() {
                Some(value) => match value.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(BifError {
                            code: 153,
                            msg: "argument is not a number".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        })
                    }
                },
                None => {
                    return Err(BifError {
                        code: 154,
                        msg: "arguments not found".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            };

            if from > to {
                return Err(BifError {
                    code: 155,
                    msg: "from > to".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }

            self.out = rng.gen_range(from..=to).to_string();
        }

        Ok(())
    }

    /*
        {:redirect; 301 >> /page :}
        {:redirect; js:reload:top >> (none) :}
    */
    fn parse_bif_redirect(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope || self.mod_negate {
            return Err(BifError {
                code: 156,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let status_code;
        let has_status_params = self.extract_params_code(true);

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.src, false);
        }

        if has_status_params {
            // When parameters are required or optional in BIF
            status_code = match self.params.as_str() {
                "301" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            code: 157,
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "301"
                }
                "302" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            code: 158,
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "302"
                }
                "303" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            code: 159,
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "303"
                }
                "307" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            code: 160,
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "307"
                }
                "308" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            code: 161,
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "308"
                }
                "js:reload:top" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_TOP.to_string();

                    "200"
                }
                "js:reload:self" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_SELF.to_string();

                    "200"
                }
                "js:redirect:top" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            code: 162,
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }
                    // TODO replace(['%2F','%3A','%3F','%3D','%26'], ['/',':','?','=','&'], url);
                    self.shared.redirect_js =
                        REDIR_JS_REDIRECT_TOP.replace("{}", &self.code).to_string();

                    "200"
                }
                "js:redirect:self" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            code: 163,
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }
                    // TODO replace(['%2F','%3A','%3F','%3D','%26'], ['/',':','?','=','&'], url);
                    self.shared.redirect_js =
                        REDIR_JS_REDIRECT_SELF.replace("{}", &self.code).to_string();

                    "200"
                }
                _ => {
                    // Parameters are optional in js:reload:self and js:reload:top
                    if !self.code.contains("js:reload:self") || !self.code.contains("js:reload:top")
                    {
                        return Err(BifError {
                            code: 164,
                            msg: "status code not allowed".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    } else {
                        "200"
                    }
                }
            };
        } else {
            // When parameters are not needed in BIF
            status_code = match self.code.as_str() {
                "js:reload:top" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_TOP.to_string();

                    "200"
                }
                "js:reload:self" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_SELF.to_string();

                    "200"
                }
                _ => {
                    return Err(BifError {
                        code: 165,
                        msg: "redirect type not allowed".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            };
        }

        self.shared.status_param = self.code.to_string();
        self.shared.status_code = status_code.to_string();

        if let Some(text) = STATUS_CODES.get(status_code) {
            self.shared.status_text = text.to_string();
        } else {
            self.shared.status_text = EMPTY_STRING;
        }

        self.shared.exit = true ^ self.mod_negate;
        self.out = EMPTY_STRING;

        Ok(())
    }

    /*
        {:replace; /from/to/ >> ... :}
        /from/to/, ~from~to~, |from|to|, ...
    */
    fn parse_bif_replace(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 166,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(false);

        let delim;
        if let Some(first_char) = self.params.chars().next() {
            delim = first_char;
        } else {
            return Err(BifError {
                code: 167,
                msg: "missing arguments".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let mut parts = self.params.split(delim);

        // discard the first
        parts.next();

        let mut from = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    code: 168,
                    msg: "arguments not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let mut to = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    code: 169,
                    msg: "arguments not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        if to.contains(BIF_OPEN) {
            to = new_child_parse!(self, &to, false);
        }

        if from.contains(BIF_OPEN) {
            from = new_child_parse!(self, &from, false);
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.out = self.code.replace(&from, &to);

        Ok(())
    }

    /*
        Play snippet:
        {:snippet; snippet-name :}

        Set snippet:
        {:snippet; snippet-name >>
            content to set
        :}
    */
    fn parse_bif_snippet(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                code: 170,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let is_set = self.extract_params_code(true);
        if is_set {
            // Set snippets in snippet files and inside snippets
            if self.inherit.current_file.contains(SNIPPETS_FILES) || self.inherit.alias == "snippet"
            {
                if self.flags.contains("|static|") {
                    self.code = new_child_parse!(self, &self.code, self.mod_scope);
                } else {
                    // required regardless of mod_scope or static
                    self.inherit.create_block_schema(&mut self.shared);
                }
                self.shared.schema["__indir"][&self.inherit.indir]["snippets"][&self.params] =
                    json!(&self.code);
                self.out = EMPTY_STRING;

                return Ok(());
            } else {
                return Err(BifError {
                    code: 171,
                    msg: "snippet cannot be set here".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }
        } else {
            // parse snippet name if need
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
            }

            self.code = get_from_key(
                &self.shared.schema["__indir"][&self.inherit.indir]["snippets"],
                &self.code,
            );
            if self.code.contains(BIF_OPEN) {
                // auto mod_scope in snippets for snippets inside snippets
                self.code = new_child_parse!(self, &self.code, self.code.contains("{:snippet;"));
            }

            self.out = self.code.to_string();

            return Ok(());
        }
    }

    /*
       {:trans; ... :}
    */
    fn parse_bif_trans(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                code: 172,
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        // For performance, we avoid calling BlockParser::new if it is not necessary
        if self.src.contains(BIF_OPEN) {
            self.src = new_child_parse!(self, &self.src, self.mod_scope);
        }

        let trans = self.get_trans(&self.src);

        // By default the input text
        if trans.is_empty() {
            if self.mod_negate {
                self.out = EMPTY_STRING;
            } else {
                self.out = self.src.clone();
            }
        } else {
            self.out = trans;
        }

        Ok(())
    }
}
