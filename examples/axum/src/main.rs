use serde_json::{json, Value};
use std::fs;
use neutralts::Template;
use axum::{
    routing::get,
    http::{StatusCode, Response},
    Router,
};

// Home
async fn home_handler() -> Response<String> {

    // A "schema" is needed for the configuration and for the data to be presented.
    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();

    // The user's language is set
    schema["inherit"]["locale"]["current"] = "en".into();

    // The theme is set.
    schema["data"]["site"]["theme"] = json!("flatly");

    // Set a framework, just as an example
    schema["data"]["current-fw"] = json!("axum");

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

        Response::builder()
            .status(status_code.parse::<u16>().unwrap())
            .header("Content-Type", "text/html")
            .body(contents)
            .unwrap()
    } else {
        Response::builder()
        .status(StatusCode::OK)  // Establecer el código de estado
        .header("Content-Type", "text/html")
        .body(contents)
        .unwrap()
    }
}

// 404
async fn handle_404() -> Response<String> {
    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();

    // Set a framework, just as an example
    schema["data"]["current-fw"] = json!("axum");

    // The theme is set.
    schema["data"]["site"]["theme"] = json!("flatly");

    // The error variables are added to the schema, just because this
    // is how we set up our custom error page.
    schema["data"]["error"] = json!({
        "code": "404",
        "text": "Not Found"
    });

    let template_path = "../../examples/www/tpl/error.ntpl";
    let mut template = Template::from_file_value(&template_path, schema).unwrap();
    let contents = template.render();

    Response::builder()
        .status(StatusCode::NOT_FOUND)  // Establecer el código de estado
        .header("Content-Type", "text/html")
        .body(contents)
        .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_handler))
        .fallback(handle_404);

    let addr = ([127, 0, 0, 1], 9090).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
