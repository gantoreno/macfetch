mod constants;

use battery::Manager;
use colored::{ColoredString, Colorize};
use core_graphics::display::{CGDisplay, CGMainDisplayID};
use iron_oxide::CGDirectDisplayCopyCurrentMetalDevice;
use libc::timeval;
use os_version::MacOS;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs};
use sysctl::Sysctl;

use crate::macfetch::utils::{ctl, host};

use super::utils::cache;

pub fn machine() -> ColoredString {
    let (username, hostname) = host::get_host_info();

    format!("{}@{}", username.bold().green(), hostname.bold().green()).normal()
}

pub fn separator() -> ColoredString {
    let (username, hostname) = host::get_host_info();
    let separator = String::from("-").repeat(username.len() + hostname.len() + 1);

    separator.normal()
}

pub fn os() -> ColoredString {
    let os = MacOS::detect()
        .map(|version| version.to_string())
        .unwrap_or_else(|_| "macOS".to_string());

    titled_segment("OS", os)
}

pub fn host() -> ColoredString {
    let host = ctl::get_ctl_info("hw.model")
        .and_then(|ctl| ctl.value_string().ok())
        .unwrap_or_else(|| "Unknown".to_string());

    titled_segment("Host", host)
}

pub fn kernel() -> ColoredString {
    let kernel = ctl::get_ctl_info("kern.osrelease")
        .and_then(|ctl| ctl.value_string().ok())
        .unwrap_or_else(|| "Unknown".to_string());

    titled_segment("Kernel", kernel)
}

pub fn uptime() -> ColoredString {
    let uptime = ctl::get_ctl_info("kern.boottime")
        .and_then(|ctl| ctl.value_as::<timeval>().ok())
        .map(|binding| {
            let time = binding.as_ref();
            let duration = Duration::new(time.tv_sec as u64, (time.tv_usec * 1000) as u32);
            let bootup_timestamp = UNIX_EPOCH + duration;
            let seconds_since_boot = SystemTime::now()
                .duration_since(bootup_timestamp)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let dd = seconds_since_boot / 60 / 60 / 24;
            let dd_suffix = if dd == 1 { "day" } else { "days" };

            let hh = seconds_since_boot / 60 / 60 % 24;
            let hh_suffix = if hh == 1 { "hour" } else { "hours" };

            let mm = seconds_since_boot / 60 % 60;
            let mm_suffix = if mm == 1 { "minute" } else { "minutes" };

            format!(
                "{} {}, {} {}, {} {}",
                dd, dd_suffix, hh, hh_suffix, mm, mm_suffix
            )
        })
        .unwrap_or_else(|| "Unavailable".to_string());

    titled_segment("Uptime", uptime)
}

pub fn packages() -> ColoredString {
    let local_packages = match fs::read_dir("/usr/local/Cellar") {
        Ok(data) => data.count(),
        Err(_) => 0,
    };
    let opt_packages = match fs::read_dir("/opt/homebrew/Cellar") {
        Ok(data) => data.count(),
        Err(_) => 0,
    };

    let packages = format!("{} (brew)", local_packages + opt_packages);

    titled_segment("Packages", packages)
}

pub fn shell() -> ColoredString {
    let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());

    titled_segment("Shell", shell)
}

pub fn resolution() -> ColoredString {
    // SAFETY: CGMainDisplayID() is a safe FFI call that returns the display ID
    // of the main display. It has no side effects and always returns a valid ID.
    let display = CGDisplay::new(unsafe { CGMainDisplayID() });

    let width = display.pixels_wide();
    let height = display.pixels_high();

    let resolution = format!("{}x{}", width, height);

    titled_segment("Resolution", resolution)
}

pub fn de() -> ColoredString {
    titled_segment("DE", "Aqua".to_string())
}

pub fn wm() -> ColoredString {
    titled_segment("WM", "Quartz Compositor".to_string())
}

pub fn terminal() -> ColoredString {
    let terminal = env::var("TERM_PROGRAM")
        .ok()
        .or_else(|| {
            env::var("ALACRITTY_WINDOW_ID")
                .ok()
                .map(|_| "Alacritty".to_string())
        })
        .or_else(|| {
            env::var("KITTY_WINDOW_ID")
                .ok()
                .map(|_| "kitty".to_string())
        })
        .unwrap_or_else(|| "unknown".to_string());

    titled_segment("Terminal", terminal)
}

pub fn cpu() -> ColoredString {
    let cpu = cache::fallback("cpu", || {
        ctl::get_ctl_info("machdep.cpu.brand_string")
            .and_then(|ctl| ctl.value_string().ok())
            .unwrap_or_else(|| "Unknown CPU".to_string())
    });

    titled_segment("CPU", cpu)
}

pub fn gpu() -> ColoredString {
    let gpu = cache::fallback("gpu", || {
        // SAFETY: CGMainDisplayID() returns the main display ID, always valid.
        // CGDisplay::new wraps it safely.
        let display = unsafe { CGDisplay::new(CGMainDisplayID()) };
        // SAFETY: CGDirectDisplayCopyCurrentMetalDevice returns a valid Metal device
        // for the given display ID. The display.id is guaranteed valid from above.
        let mtl_device = unsafe { CGDirectDisplayCopyCurrentMetalDevice(display.id) };
        // SAFETY: get_name() returns a valid string reference from the Metal device.
        unsafe { mtl_device.get_name().to_string() }
    });

    titled_segment("GPU", gpu)
}

pub fn battery() -> ColoredString {
    let battery_info = (|| -> Option<String> {
        let manager = Manager::new().ok()?;
        let battery = manager.batteries().ok()?.next()?.ok()?;
        let percentage = battery.state_of_charge().value * 100.0;
        Some(format!("{}%", percentage.round()))
    })();

    match battery_info {
        Some(info) => titled_segment("Power Source", format!("Battery: {}", info)),
        None => titled_segment("Power Source", "Plugged In".to_string()),
    }
}

pub fn memory() -> ColoredString {
    let system_memory_info = system_info::mem::SystemMemory::new();

    let total_bytes = system_memory_info.total as f64;
    let avail_bytes = system_memory_info.avail as f64;
    let used_bytes = total_bytes - avail_bytes;

    let to_gib = 1024.00_f64 * 1024.00_f64 * 1024.00_f64;
    let total_gib = total_bytes / to_gib;
    let used_gib = used_bytes / to_gib;

    let percentage = (used_bytes / total_bytes) * 100.0;
    let memory = format!(
        "{:.2} GiB / {:.2} GiB ({}%)",
        used_gib,
        total_gib,
        percentage.round()
    );

    titled_segment("Memory", memory)
}

pub fn dark_colors() -> ColoredString {
    let mut blocks = String::new();

    for i in 40..=47 {
        blocks.push_str(&format!("\x1b[{}m   ", i));
    }

    blocks.push_str("\x1b[0m");
    blocks.normal()
}

pub fn light_colors() -> ColoredString {
    let mut blocks = String::new();

    for i in 100..=107 {
        blocks.push_str(&format!("\x1b[{}m{}", i, constants::COLOR_BLOCK_CHARACTER));
    }

    blocks.push_str("\x1b[0m");
    blocks.normal()
}

pub fn empty() -> ColoredString {
    "".normal()
}

fn titled_segment(name: &str, out: String) -> ColoredString {
    format!("{}: {}", name.bold().yellow(), out).normal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_returns_empty_string() {
        let result = empty();
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn test_de_returns_aqua() {
        let result = de();
        assert!(result.to_string().contains("Aqua"));
    }

    #[test]
    fn test_wm_returns_quartz() {
        let result = wm();
        assert!(result.to_string().contains("Quartz Compositor"));
    }

    #[test]
    fn test_titled_segment_formatting() {
        let result = titled_segment("Test", "Value".to_string());
        let plain = result.to_string();
        assert!(plain.contains("Test"));
        assert!(plain.contains("Value"));
        assert!(plain.contains(":"));
    }

    #[test]
    fn test_dark_colors_has_reset_code() {
        let result = dark_colors();
        let plain = result.to_string();
        assert!(plain.contains("\x1b[0m"));
    }

    #[test]
    fn test_light_colors_has_reset_code() {
        let result = light_colors();
        let plain = result.to_string();
        assert!(plain.contains("\x1b[0m"));
    }
}
