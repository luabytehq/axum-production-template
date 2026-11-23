use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use std::time::Instant;

pub async fn track_execution_time(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let mut response = next.run(req).await;
    let elapsed = start.elapsed().as_micros();

    if let Ok(val) = HeaderValue::from_str(&elapsed.to_string()) {
        response
            .headers_mut()
            .insert("x-execution-time-micros", val);
    }

    response
}
