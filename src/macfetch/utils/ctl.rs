use sysctl::{Ctl, Sysctl};

pub fn get_ctl_info(key: &str) -> Ctl {
    let ctl = Ctl::new(key).ok().unwrap();

    return ctl;
}
