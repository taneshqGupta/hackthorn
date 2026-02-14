use axum::body::Body;
use axum::http::{HeaderValue, Request, Response};
use axum::middleware::Next;

pub async fn add_partitioned_attribute(request: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    let mut modified_cookies = Vec::new();

    for (name, value) in headers.iter() {
        if name.as_str().to_lowercase() == "set-cookie" {
            if let Ok(cookie_str) = value.to_str() {
                if cookie_str.contains("aegis_session") {
                    // Remove Secure flag, use SameSite=None for cross-origin support
                    let mut cookie = cookie_str
                        .replace("; Secure", "")
                        .replace(";Secure", "")
                        .replace("; SameSite=Lax", "")
                        .replace(";SameSite=Lax", "")
                        .replace("; SameSite=None", "")
                        .replace(";SameSite=None", "");
                    
                    // SameSite=None without Secure - works for both HTTP and HTTPS
                    cookie = format!("{}; SameSite=None", cookie);
                    tracing::info!("Cookie set: {}", cookie);
                    
                    modified_cookies.push(cookie);
                } else {
                    modified_cookies.push(cookie_str.to_string());
                }
            }
        }
    }

    headers.remove("set-cookie");

    for cookie_str in modified_cookies {
        if let Ok(header_value) = HeaderValue::from_str(&cookie_str) {
            headers.append("set-cookie", header_value);
        }
    }

    response
}
