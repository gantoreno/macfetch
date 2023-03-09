use std::fs;

pub fn fallback(name: &str, callback: fn() -> String) -> String {
    let cached_value = match fs::read_to_string(format!("/Library/Caches/macfetch/{}", name)) {
        Ok(data) => data.trim_end().to_string(),
        Err(_) => "".to_string(),
    };

    if !cached_value.is_empty() {
        return cached_value;
    }

    let value = callback();

    fs::create_dir_all("/Library/Caches/macfetch").ok();
    fs::write(format!("/Library/Caches/macfetch/{}", name), value).ok();

    return callback();
}
