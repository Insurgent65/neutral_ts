use std::collections::HashMap;
use lazy_static::lazy_static;

///  Bif delimiters
///
/// ```plaintext
///
///   .------------------------------> BIF_OPEN = { + BIF_DELIM
///   |.-----------------------------> BIF_DELIM
///   ||.----------------------------> BIF_MOD_... (Modifiers)
///   |||       .--------------------> BIF_NAME
///   |||       |    .---------------> BIF_ARRAY
///   |||       |    ||    .---------> BIF_CODE
///   |||       |    ||    ||      .-> BIF_CLOSE = BIF_DELIM + }
///   |||       |    ||    ||      |
///   vvv       v    vv    vv      v
///   {:!snippet; var->key >> ... :}
///

pub const BIF_DELIM: &str = ":";
pub const BIF_OPEN: &str = "{:";
pub const BIF_CLOSE: &str = ":}";
pub const BIF_NAME: &str = ";";
pub const BIF_CODE: &str = ">>";
pub const BIF_ARRAY: &str = "->";
pub const BIF_COMMENT: &str = "*";
pub const BIF_COMMENT_OPEN: &str = "{:*";
pub const BIF_COMMENT_CLOSE: &str = "*:}";
pub const BIF_MOD_FILTER: &str = "&";
pub const BIF_MOD_NEGATE: &str = "!";
pub const BIF_MOD_UPLINE: &str = "^";
pub const BIF_MOD_SCOPE: &str = "+";

pub const BIF_DELIM_B: &[u8] = BIF_DELIM.as_bytes();
pub const BIF_OPEN_B: &[u8] = BIF_OPEN.as_bytes();
pub const BIF_OPEN0: u8 = BIF_OPEN.as_bytes()[0];
pub const BIF_OPEN1: u8 = BIF_OPEN.as_bytes()[1];
pub const BIF_CLOSE_B: &[u8] = BIF_CLOSE.as_bytes();
pub const BIF_CLOSE0: u8 = BIF_CLOSE.as_bytes()[0];
pub const BIF_CLOSE1: u8 = BIF_CLOSE.as_bytes()[1];
pub const BIF_CODE_B: &[u8] = BIF_CODE.as_bytes();
pub const BIF_COMMENT_B: u8 = BIF_COMMENT.as_bytes()[0];
pub const BIF_COMMENT_OPEN_B: &[u8] = BIF_COMMENT_OPEN.as_bytes();
pub const BIF_COMMENT_CLOSE_B: &[u8] = BIF_COMMENT_CLOSE.as_bytes();

pub const BIF_SANITIZE_OPEN: &str = "&#123;:";
pub const BIF_SANITIZE_CLOSE: &str = ":&#125;";

pub const UNPRINTABLE: &str = "&#0;";
pub const NULL: &str = "&#0;";
pub const EMPTY: &str = "";
pub const EMPTY_STRING: String = String::new();
pub const SPACE: &str = "&#160;";
pub const CRLF: &str = "&#10;";
pub const BACKSPACE: &str = "&#9224";
pub const FALSE: bool = false;
pub const TRUE: bool = true;

pub const BIF_ALLOWED: [&str; 2] = ["{:allow;", "{:!allow;"];
pub const SNIPPETS_FILES: &str = "snippet";

pub const BIF_LIST: [&str; 30] = [
    "",
    "allow",
    "array",
    "bool",
    "coalesce",
    "code",
    "count",
    "data",
    "date",
    "declare",
    "defined",
    "each",
    "else",
    "eval",
    "exit",
    "filled",
    "flg",
    "for",
    "hash",
    "include",
    "lang",
    "locale",
    "moveto",
    "neutral",
    "param",
    "rand",
    "redirect",
    "replace",
    "snippet",
    "trans",
];

pub const BIF_ALIAS_LIST: [&str; 31] = [
    "allow",
    "array",
    "bool",
    "coalesce",
    "code",
    "count",
    "data",
    "date",
    "declare",
    "defined",
    "each",
    "else",
    "eval",
    "exit",
    "filled",
    "flg",
    "for",
    "hash",
    "include",
    "lang",
    "locale",
    "moveto",
    "neutral",
    "param",
    "rand",
    "redirect",
    "replace",
    "snippet",
    "trans",
    "unprintable",
    "var",
];

lazy_static! {
    pub static ref STATUS_CODES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("100", "Continue");
        m.insert("101", "Switching Protocols");
        m.insert("103", "Early Hints");
        m.insert("200", "OK");
        m.insert("201", "Created");
        m.insert("202", "Accepted");
        m.insert("203", "Non-Authoritative Information");
        m.insert("204", "No Content");
        m.insert("205", "Reset Content");
        m.insert("206", "Partial Content");
        m.insert("208", "Already Reported");
        m.insert("226", "IM Used");
        m.insert("300", "Multiple Choices");
        m.insert("301", "Moved Permanently");
        m.insert("302", "Found");
        m.insert("303", "See Other");
        m.insert("304", "Not Modified");
        m.insert("305", "Use Proxy");
        m.insert("306", "Switch Proxy"); // old http version
        m.insert("307", "Temporary Redirect");
        m.insert("308", "Permanent Redirect");
        m.insert("400", "Bad Request");
        m.insert("401", "Unauthorized");
        m.insert("402", "Payment Required");
        m.insert("403", "Forbidden");
        m.insert("404", "Not Found");
        m.insert("405", "Method Not Allowed");
        m.insert("406", "Not Acceptable");
        m.insert("407", "Proxy Authentication Required");
        m.insert("408", "Request Time-out");
        m.insert("409", "Conflict");
        m.insert("410", "Gone");
        m.insert("411", "Length Required");
        m.insert("412", "Precondition Failed");
        m.insert("413", "Payload Too Large");
        m.insert("414", "URI Too Long");
        m.insert("415", "Unsupported Media Type");
        m.insert("416", "Range Not Satisfiable");
        m.insert("417", "Expectation Failed");
        m.insert("421", "Misdirected Request");
        m.insert("422", "Unprocessable Entity");
        m.insert("423", "Locked");
        m.insert("424", "Failed Dependency");
        m.insert("425", "Too Early");
        m.insert("426", "Upgrade Required");
        m.insert("428", "Precondition Required");
        m.insert("429", "Too Many Requests");
        m.insert("431", "Request Header Fields Too Large");
        m.insert("451", "Unavailable For Legal Reasons");
        m.insert("500", "Internal Server Error");
        m.insert("501", "Not Implemented");
        m.insert("502", "Bad Gateway");
        m.insert("503", "Service Unavailable");
        m.insert("504", "Gateway Time-out");
        m.insert("505", "HTTP Version Not Supported");
        m.insert("506", "Variant Also Negotiates (Experimental)");
        m.insert("510", "Not Extended");
        m.insert("511", "Network Authentication Required");

        m
    };
}

pub const REDIR_JS_RELOAD_TOP: &str = "<!DOCTYPE html><script>top.location.href=self.location.href.split('#')[0];</script>";
pub const REDIR_JS_RELOAD_SELF: &str = "<!DOCTYPE html><script>self.location.href=self.location.href.split('#')[0]</script>";
pub const REDIR_JS_REDIRECT_TOP: &str = "<!DOCTYPE html><script>top.location.href='{}';</script>";
pub const REDIR_JS_REDIRECT_SELF: &str = "<!DOCTYPE html><script>self.location.href='{}';</script>";
