use std::fs;

/// Cache directory path for storing cached values
const CACHE_DIR: &str = "/Library/Caches/macfetch";

/// Attempts to read a cached value, or computes and caches it using the callback
pub fn fallback(name: &str, callback: fn() -> String) -> String {
    let cache_path = format!("{}/{}", CACHE_DIR, name);

    if let Ok(data) = fs::read_to_string(&cache_path) {
        let cached_value = data.trim_end().to_string();
        if !cached_value.is_empty() {
            return cached_value;
        }
    }

    let value = callback();

    fs::create_dir_all(CACHE_DIR).ok();
    fs::write(&cache_path, &value).ok();

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_fallback_computes_value_when_no_cache() {
        let test_name = "test_no_cache";
        let cache_path = format!("{}/{}", CACHE_DIR, test_name);

        // Clean up any existing cache
        let _ = fs::remove_file(&cache_path);

        fn compute_value() -> String {
            "computed_value".to_string()
        }

        let result = fallback(test_name, compute_value);
        assert_eq!(result, "computed_value");

        // Clean up
        let _ = fs::remove_file(&cache_path);
    }

    #[test]
    fn test_fallback_returns_cached_value() {
        let test_name = "test_cached";
        let cache_path = format!("{}/{}", CACHE_DIR, test_name);

        // Set up cache directory and file
        let _ = fs::create_dir_all(CACHE_DIR);
        let _ = fs::write(&cache_path, "cached_value");

        fn should_not_be_called() -> String {
            panic!("Callback should not be called when cache exists");
        }

        let result = fallback(test_name, should_not_be_called);
        assert_eq!(result, "cached_value");

        // Clean up
        let _ = fs::remove_file(&cache_path);
    }

    #[test]
    fn test_fallback_creates_cache_file() {
        let test_name = "test_creates_cache";
        let cache_path = format!("{}/{}", CACHE_DIR, test_name);

        // Clean up any existing cache
        let _ = fs::remove_file(&cache_path);

        fn compute_value() -> String {
            "new_value".to_string()
        }

        let _ = fallback(test_name, compute_value);

        // Verify cache file was created
        assert!(Path::new(&cache_path).exists());

        // Verify content
        let content = fs::read_to_string(&cache_path).unwrap();
        assert_eq!(content, "new_value");

        // Clean up
        let _ = fs::remove_file(&cache_path);
    }
}
