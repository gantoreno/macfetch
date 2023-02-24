use std::{fs, process::Command};

use base64::{engine::general_purpose, Engine};

pub fn get_command_output(command: &str) -> String {
    let stdout = Command::new("/bin/sh")
        .args(["-c", command])
        .output()
        .expect("Unknown")
        .stdout;
    let output = String::from_utf8_lossy(&stdout).trim_end().to_string();

    return output;
}

pub fn get_command_output_cached(command: &str) -> String {
    let cache_key = general_purpose::STANDARD.encode(command);
    let cached_value = match fs::read_to_string(format!("/Library/Caches/macfetch/{}", cache_key)) {
        Ok(data) => data.trim_end().to_string(),
        Err(_) => "".to_string(),
    };

    if !cached_value.is_empty() {
        return cached_value;
    }

    let value = get_command_output(command);

    fs::create_dir_all("/Library/Caches/macfetch").ok();
    fs::write(format!("/Library/Caches/macfetch/{}", cache_key), value).ok();

    return get_command_output(command);
}
