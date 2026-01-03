use colored::ColoredString;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::macfetch::segments;

#[derive(Deserialize, Default)]
pub struct GeneralConfig {
    pub segments: Option<Vec<String>>,
}

#[derive(Deserialize, Default)]
pub struct Config {
    pub general: Option<GeneralConfig>,
}

fn config_path() -> Option<PathBuf> {
    env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".config/macfetch/config.toml"))
}

fn load_config() -> Option<Config> {
    let path = config_path()?;
    let content = fs::read_to_string(&path).ok()?;

    match toml::from_str(&content) {
        Ok(config) => Some(config),
        Err(e) => {
            eprintln!("Warning: Failed to parse config file: {}", e);
            None
        }
    }
}

fn segment_registry() -> HashMap<&'static str, fn() -> ColoredString> {
    let mut map: HashMap<&'static str, fn() -> ColoredString> = HashMap::new();

    map.insert("machine", segments::machine);
    map.insert("separator", segments::separator);
    map.insert("os", segments::os);
    map.insert("host", segments::host);
    map.insert("kernel", segments::kernel);
    map.insert("uptime", segments::uptime);
    map.insert("packages", segments::packages);
    map.insert("shell", segments::shell);
    map.insert("resolution", segments::resolution);
    map.insert("de", segments::de);
    map.insert("wm", segments::wm);
    map.insert("terminal", segments::terminal);
    map.insert("cpu", segments::cpu);
    map.insert("gpu", segments::gpu);
    map.insert("battery", segments::battery);
    map.insert("memory", segments::memory);
    map.insert("empty", segments::empty);
    map.insert("dark_colors", segments::dark_colors);
    map.insert("light_colors", segments::light_colors);

    map
}

pub fn default_segments() -> Vec<fn() -> ColoredString> {
    vec![
        segments::machine,
        segments::separator,
        segments::os,
        segments::host,
        segments::kernel,
        segments::uptime,
        segments::packages,
        segments::shell,
        segments::resolution,
        segments::de,
        segments::wm,
        segments::terminal,
        segments::cpu,
        segments::gpu,
        segments::battery,
        segments::memory,
        segments::empty,
        segments::dark_colors,
        segments::light_colors,
    ]
}

fn resolve_segments(names: &[String]) -> Vec<fn() -> ColoredString> {
    let registry = segment_registry();
    let mut segments = Vec::new();

    for name in names {
        match registry.get(name.as_str()) {
            Some(&func) => segments.push(func),
            None => eprintln!("Warning: Unknown segment '{}', skipping", name),
        }
    }

    segments
}

pub fn get_segments() -> Vec<fn() -> ColoredString> {
    match load_config() {
        Some(config) => match config.general {
            Some(general) => match general.segments {
                Some(names) if !names.is_empty() => resolve_segments(&names),
                _ => default_segments(),
            },
            None => default_segments(),
        },
        None => default_segments(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_segments_valid() {
        let names = vec!["os".to_string(), "cpu".to_string()];
        let segments = resolve_segments(&names);
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_resolve_segments_invalid_skipped() {
        let names = vec![
            "os".to_string(),
            "invalid_segment".to_string(),
            "cpu".to_string(),
        ];
        let segments = resolve_segments(&names);
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_default_segments_count() {
        let segments = default_segments();
        assert_eq!(segments.len(), 19);
    }

    #[test]
    fn test_segment_registry_has_all_segments() {
        let registry = segment_registry();
        assert_eq!(registry.len(), 19);
        assert!(registry.contains_key("machine"));
        assert!(registry.contains_key("light_colors"));
    }
}
