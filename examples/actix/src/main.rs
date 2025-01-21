use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde_json::{json, Value};
use std::fs;
use neutralts::Template;


#[get("/")]
async fn home(req: actix_web::HttpRequest) -> impl Responder {

    // A "schema" is needed for the configuration and for the data to be presented.
    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();

    // The user's language is set
    let accepted = join_array_elements(&schema["data"]["site"]["languages"]);
    schema["inherit"]["locale"]["current"] = json!(negociate_language(&req, &accepted, "en"));

    // The theme is set. Any value coming from the context (env, cookies, ...)
    // should be considered unsafe, here we will ignore it as an example of
    // how Neutral can handle this.
    schema["data"]["site"]["theme"] = json!(get_theme(&req));

    // Set a framework, just as an example
    schema["data"]["current-fw"] = json!("actix-web");

    // Create the template
    let template_path = "../../examples/www/tpl/home.ntpl";
    let mut template = Template::from_file_value(&template_path, schema).unwrap();

    // Rendered content
    let contents = template.render();

    // If “exit” or “redirect” is used, the status codes must be managed.
    let status_code = template.get_status_code().clone();
    let status_text = template.get_status_text().clone();

    // Only in certain cases, e.g., redirect.
    let _status_param = template.get_status_param().clone();

    // If not changed (with "{:exit;:}" for example) the template always
    // returns a status code 200 OK.
    if status_code.as_str() >= "400" {
        let error = json!({
            "data": {
                "error": {
                    "code": status_code,
                    "text": status_text
                }
            }
        });

        // The custom error page is used.
        template.set_src_path("../../examples/www/tpl/error.ntpl").unwrap();

        // The error variables are added to the schema, just because this
        // is how we set up our custom error page.
        template.merge_schema_value(error);

        // Rendered content for error custom page.
        // Be careful not to re-render the content that causes the error,
        // for example if the error occurs in a snippet that shares the error page.
        let contents = template.render();

        // The response is sent with the corresponding error.
        // I'm sure there is a better way to do this, just as an example.
        return match status_code.as_str() {
            "401" => HttpResponse::Unauthorized().body(contents),
            "403" => HttpResponse::Forbidden().body(contents),
            "404" => HttpResponse::NotFound().body(contents),
            "503" => HttpResponse::ServiceUnavailable().body(contents),
            // ...
            _ => HttpResponse::InternalServerError().body(contents),
        }
    } else {
        HttpResponse::Ok().content_type("text/html").body(contents)
    }
}

async fn not_found(req: actix_web::HttpRequest) -> impl Responder {
    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();
    let accepted = join_array_elements(&schema["data"]["site"]["languages"]);
    schema["inherit"]["locale"]["current"] = json!(negociate_language(&req, &accepted, "en"));
    schema["data"]["site"]["theme"] = json!(get_theme(&req));

    // The error variables are added to the schema, just because this
    // is how we set up our custom error page.
    schema["data"]["error"] = json!({
        "code": "404",
        "text": "Not Found"
    });

    let template_path = "../../examples/www/tpl/error.ntpl";
    let mut template = Template::from_file_value(&template_path, schema).unwrap();
    let contents = template.render();

    HttpResponse::NotFound().body(contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}


// -----------------
// Utility functions
// -----------------

fn negociate_language(
    req: &HttpRequest,
    accepted_languages_str: &str,
    default_language: &str,
) -> String {
    let query_string = req.query_string();
    let accept_language = req.headers().get("Accept-Language").map(|v| v.to_str().unwrap()).unwrap_or("");
    let accepted_languages: Vec<String> = accepted_languages_str.split_whitespace().map(String::from).collect();

    if !query_string.is_empty() {
        for pair in query_string.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                if key == "lang" && accepted_languages.contains(&value.to_string()) {
                    return value.to_string();
                }
            }
        }
    }

    if let Some(cookie) = req.cookie("lang") {
        return cookie.value().to_string();
    }

    if !accept_language.is_empty() {
        let mut languages: Vec<_> = accept_language.split(',')
            .map(|lang| {
                let parts: Vec<&str> = lang.trim().split(';').collect();
                let primary_lang = parts[0];
                let quality = parts.get(1)
                    .and_then(|q| q.strip_prefix("q="))
                    .and_then(|v| v.parse::<f32>().ok())
                    .unwrap_or(1.0);
                (primary_lang, quality)
            })
            .collect();
        languages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for &(lang, _) in &languages {
            if let Some((primary_lang, _)) = lang.split_once('-') {
                if accepted_languages.contains(&primary_lang.to_string()) {
                    return primary_lang.to_string();
                }
            } else if accepted_languages.contains(&lang.to_string()) {
                return lang.to_string();
            }
        }
    }

    default_language.to_string()
}

fn get_theme_from_query(req: &HttpRequest) -> Option<String> {
    let query_string = req.query_string();
    for pair in query_string.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == "theme" {
                return Some(value.to_string());
            }
        }
    }

    None
}

fn get_theme_from_cookies(req: &HttpRequest) -> Option<String> {
    if let Some(cookie) = req.cookie("theme") {
        return Some(cookie.value().to_string());
    }

    None
}

fn get_theme(req: &HttpRequest) -> String {
    if let Some(theme_from_query) = get_theme_from_query(req) {
        return theme_from_query;
    }
    if let Some(theme_from_cookies) = get_theme_from_cookies(req) {
        return theme_from_cookies;
    }

    "sketchy".to_string()
}

fn join_array_elements(value: &Value) -> String {
    match value {
        Value::Array(arr) => arr.iter()
                                  .filter_map(|v| v.as_str())
                                  .collect::<Vec<&str>>()
                                  .join(" "),
        Value::Object(obj) => obj.values()
                                   .flat_map(|v| join_array_elements(v).into_bytes())
                                   .map(char::from)
                                   .collect(),
        _ => "".to_string(),
    }
}
