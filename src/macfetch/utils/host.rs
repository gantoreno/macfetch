use cached::proc_macro::cached;
use std::borrow::Borrow;
use sys_info::hostname;
use users::{Users, UsersCache};

#[cached]
pub fn get_host_info() -> (String, String) {
    let cache = UsersCache::new();
    let uid = cache.get_current_uid();

    let user_binding = cache.get_user_by_uid(uid).unwrap();
    let username = match user_binding.name().to_str() {
        Some(name) => name,
        None => "",
    };

    let binding = hostname();
    let hostname = match binding.borrow() {
        Ok(host) => host.as_str(),
        Err(_) => "",
    };

    return (username.to_string(), hostname.to_string());
}
