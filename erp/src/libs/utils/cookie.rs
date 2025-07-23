use std::collections::HashMap;

use cookie::Cookie;

pub fn parse_cookie(cookie_header: &str) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    for cookie_str in cookie_header.split(';') {
        let cookie_str = cookie_str.trim();
        if let Ok(cookie) = Cookie::parse(cookie_str) {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }
    }
    cookies
}

