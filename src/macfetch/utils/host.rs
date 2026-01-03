use cached::proc_macro::cached;
use sys_info::hostname;
use users::{Users, UsersCache};

#[cached]
pub fn get_host_info() -> (String, String) {
    let cache = UsersCache::new();
    let uid = cache.get_current_uid();

    let username = cache
        .get_user_by_uid(uid)
        .and_then(|user| user.name().to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string());

    let hostname = hostname().unwrap_or_else(|_| "unknown".to_string());

    (username, hostname)
}
