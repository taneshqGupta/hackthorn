use axum::body::Body;
use axum::http::{HeaderValue, Request, Response};
use axum::middleware::Next;

pub async fn add_partitioned_attribute(request: Request<Body>, next: Next) -> Response<Body> {
    // Check if request is from localhost
    let is_localhost = request
        .headers()
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .map(|referer| referer.contains("localhost") || referer.contains("127.0.0.1"))
        .unwrap_or(false);
    
    tracing::info!("Cookie middleware: is_localhost={}", is_localhost);
    
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    let mut modified_cookies = Vec::new();

    for (name, value) in headers.iter() {
        if name.as_str().to_lowercase() == "set-cookie" {
            if let Ok(cookie_str) = value.to_str() {
                tracing::info!("Original cookie: {}", cookie_str);
                
                if cookie_str.contains("aegis_session") {
                    // Remove existing SameSite and Secure attributes
                    let mut cookie = cookie_str
                        .replace("; Secure", "")
                        .replace(";Secure", "")
                        .replace("; SameSite=None", "")
                        .replace(";SameSite=None", "")
                        .replace("; SameSite=Lax", "")
                        .replace(";SameSite=Lax", "");
                    
                    if is_localhost {
                        // Localhost: Use Lax without Secure for HTTP
                        cookie = format!("{}; SameSite=Lax", cookie);
                        tracing::info!("Modified for localhost: {}", cookie);
                    } else {
                        // Production: Use None with Secure for HTTPS cross-origin
                        cookie = format!("{}; SameSite=None; Secure; Partitioned", cookie);
                        tracing::info!("Modified for production: {}", cookie);
                    }
                    
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
