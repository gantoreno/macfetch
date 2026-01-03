use sysctl::{Ctl, Sysctl};

pub fn get_ctl_info(key: &str) -> Option<Ctl> {
    Ctl::new(key).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ctl_info_valid_key() {
        // kern.osrelease should exist on all macOS systems
        let result = get_ctl_info("kern.osrelease");
        assert!(result.is_some());
    }

    #[test]
    fn test_get_ctl_info_invalid_key() {
        let result = get_ctl_info("invalid.nonexistent.key");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_ctl_info_can_read_value() {
        let ctl = get_ctl_info("kern.osrelease").unwrap();
        let value = ctl.value_string();
        assert!(value.is_ok());
        assert!(!value.unwrap().is_empty());
    }
}
