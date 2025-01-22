use rocket::{get, routes, http::CookieJar};
use rocket::{catch, http::Status};
use rocket::response::{status::Custom, content::RawHtml};
use serde_json::{json, Value};
use std::fs;
use neutralts::Template;

#[macro_use]
extern crate rocket;

#[get("/?<theme>&<lang>")]
fn home(theme: Option<String>, lang: Option<String>, cookies: &CookieJar<'_>) -> Custom<RawHtml<String>> {

    // A "schema" is needed for the configuration and for the data to be presented.
    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();

    // The user's language is set
    if let Some(lang) = lang {
        schema["inherit"]["locale"]["current"] = json!(lang);
    } else if let Some(lang) = cookies.get("lang").map(|cookie| cookie.value().to_string()) {
        schema["inherit"]["locale"]["current"] = json!(lang);
    } else {
        schema["inherit"]["locale"]["current"] = "en".into();
    }

    // The theme is set. Any value coming from the context (env, cookies, ...)
    // should be considered unsafe, here we will ignore it as an example of
    // how Neutral can handle this.
    if let Some(theme) = theme {
        schema["data"]["site"]["theme"] = json!(theme);
    } else if let Some(theme) = cookies.get("theme").map(|cookie| cookie.value().to_string()) {
        schema["data"]["site"]["theme"] = json!(theme);
    } else {
        schema["data"]["site"]["theme"] = "sketchy".into();
    }

    // Set a framework, just as an example
    schema["data"]["current-fw"] = json!("rocket");

    // Create the template
    let template_path = "../../examples/www/tpl/home.ntpl";
    let mut template = Template::from_file_value(&template_path, schema.clone()).unwrap();

    // Rendered content
    let contents = template.render();

    // If “exit” or “redirect” is used, the status codes must be managed.
    let status_code = template.get_status_code().clone();
    let status_text = template.get_status_text().clone();

    // Only in certain cases, e.g., redirect.
    let _status_param = template.get_status_param().clone();

    // Convertir el string a u16
    let status_u16: u16 = status_code.parse().unwrap();  // Puede fallar si el string no es válido

    // Crear el estado con el código numérico
    let status = Status::from_code(status_u16).unwrap();

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
        let error_contents = template.render();

        // The response is sent with the corresponding error.
        Custom(status, RawHtml(error_contents))
    } else {
        Custom(status, RawHtml(contents))
    }
}

#[catch(404)]
fn not_found() -> Custom<RawHtml<String>> {
    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();

    schema["data"]["current-fw"] = json!("rocket");

    // The error variables are added to the schema, just because this
    // is how we set up our custom error page.
    schema["data"]["error"] = json!({
        "code": "404",
        "text": "Not Found"
    });

    let template_path = "../../examples/www/tpl/error.ntpl";
    let mut template = Template::from_file_value(&template_path, schema.clone()).unwrap();

    let contents = template.render();

    Custom(Status::NotFound, RawHtml(contents))
}

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::Config {
        address: "127.0.0.1".parse().unwrap(),
        port: 9090,
        ..Default::default()
    })
    .mount("/", routes![home])
    .register("/", catchers![not_found])
}
