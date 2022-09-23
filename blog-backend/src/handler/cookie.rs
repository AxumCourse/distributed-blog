use axum::http::HeaderMap;

pub fn get(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookie = headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());
    match cookie {
        Some(cookie) => {
            let cookie = cookie.as_str();
            let cs: Vec<&str> = cookie.split(';').collect();
            for item in cs {
                let item: Vec<&str> = item.split('=').collect();
                if item.len() != 2 {
                    continue;
                }
                let key = item[0];
                let val = item[1];
                let key = key.trim();
                let val = val.trim();
                if key == name {
                    return Some(val.to_string());
                }
            }
            None
        }
        None => None,
    }
}
