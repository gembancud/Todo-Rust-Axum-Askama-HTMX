use axum::{
    body::Body,
    extract::Request,
    http::{self, header, Response},
    middleware::Next,
};

#[derive(Clone)]
enum HTMXTriggerMarker {
    AddTodo,
}
impl HTMXTriggerMarker {
    fn as_str(&self) -> &'static str {
        match self {
            HTMXTriggerMarker::AddTodo => "clearInput",
        }
    }
}

pub async fn htmx_trigger_middleware(request: Request<Body>, next: Next) -> Response<Body> {
    // Clone the HTTP method and URI from the request.
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Run the next middleware or handler in the stack and await its response.
    let mut response = next.run(request).await;

    // Clone the status code from the response.
    let status = response.status().clone();

    // Get a mutable reference to the headers of the response.
    let headers = response.headers_mut();

    // If the status code of the response is OK (200),
    if status == http::StatusCode::OK {
        // Match on the method and path of the request.
        match (method, uri.path()) {
            // If the method is POST and the path is "/",
            (http::Method::POST, "/") => {
                // Get the string representation of the HTMXTriggerMarker::AddTodo enum variant.
                let trigger = HTMXTriggerMarker::AddTodo.as_str();

                // Insert a new header into the response headers.
                // The header name is "HX-Trigger" and the value is "htmx-trigger={trigger}".
                headers.insert(
                    "HX-Trigger",
                    header::HeaderValue::from_str(&trigger).unwrap(),
                );
            }

            // For all other combinations of method and path, do nothing.
            _ => {}
        }
    }

    // Return the (possibly modified) response.
    response
}
