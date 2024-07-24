use axum::http::HeaderMap;

const MINUTES: u64 = 60;
const HOURS: u64 = 60 * MINUTES;
const DAYS: u64 = 24 * HOURS;
const WEEKS: u64 = 7 * DAYS;
const YEARS: u64 = 365 * DAYS;

pub fn generate_header_with_age(age: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let calculated_age = match age.to_lowercase().as_str() {
        s if s.ends_with("s") => s.trim_end_matches("s").parse::<u64>().unwrap(),
        m if m.ends_with("m") => m.trim_end_matches("m").parse::<u64>().unwrap() * MINUTES,
        h if h.ends_with("h") => h.trim_end_matches("h").parse::<u64>().unwrap() * HOURS,
        d if d.ends_with("d") => d.trim_end_matches("d").parse::<u64>().unwrap() * DAYS,
        w if w.ends_with("w") => w.trim_end_matches("w").parse::<u64>().unwrap() * WEEKS,
        y if y.ends_with("y") => y.trim_end_matches("y").parse::<u64>().unwrap() * YEARS,
        _ => 0,
    };

    println!("calculated_age: {}", calculated_age);

    let cache_age = if calculated_age <= 0 {
        "no-cache".to_string()
    } else {
        format!("public, max-age={}", calculated_age)
    };

    headers.insert("Cache-Control", cache_age.parse().unwrap());

    headers
}
