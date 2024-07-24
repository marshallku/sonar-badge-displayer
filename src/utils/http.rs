use axum::http::HeaderMap;

pub fn generate_header_with_age(age: u32) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let cache_age = if age <= 0 {
        "no-cache".to_string()
    } else {
        format!("public, max-age={}", age)
    };

    headers.insert("Cache-Control", cache_age.parse().unwrap());

    headers
}
