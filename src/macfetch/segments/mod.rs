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

pub fn machine() -> ColoredString {
    let (username, hostname) = host::get_host_info();

    return format!("{}@{}", username.bold().green(), hostname.bold().green()).normal();
}

pub fn separator() -> ColoredString {
    let (username, hostname) = host::get_host_info();
    let separator = String::from("-").repeat(username.len() + hostname.len() + 1);

    return separator.normal();
}

pub fn os() -> ColoredString {
    let os = MacOS::detect().ok().unwrap().to_string();

    return titled_segment("OS", os);
}

pub fn host() -> ColoredString {
    let host = ctl::get_ctl_info("hw.model").value_string().ok().unwrap();

    return titled_segment("Host", host);
}

pub fn kernel() -> ColoredString {
    let kernel = ctl::get_ctl_info("kern.osrelease")
        .value_string()
        .ok()
        .unwrap();

    return titled_segment("Kernel", kernel);
}

pub fn uptime() -> ColoredString {
    let binding = ctl::get_ctl_info("kern.boottime")
        .value_as::<timeval>()
        .unwrap();
    let time = binding.as_ref();

    let duration = Duration::new(time.tv_sec as u64, (time.tv_usec * 1000) as u32);
    let bootup_timestamp = UNIX_EPOCH + duration;
    let seconds_since_boot = match SystemTime::now().duration_since(bootup_timestamp) {
        Ok(data) => data.as_secs(),
        Err(_) => 0,
    };

    let mut uptime = String::from("");

    let dd = seconds_since_boot / 60 / 60 / 24;
    let dd_suffix = if dd == 1 { "day" } else { "days" };

    let hh = seconds_since_boot / 60 / 60 % 24;
    let hh_suffix = if hh == 1 { "hour" } else { "hours" };

    let mm = seconds_since_boot / 60 % 60;
    let mm_suffix = if mm == 1 { "minute" } else { "minutes" };

    uptime.push_str(String::from(format!("{} {}, ", dd, dd_suffix)).as_str());
    uptime.push_str(String::from(format!("{} {}, ", hh, hh_suffix)).as_str());
    uptime.push_str(String::from(format!("{} {}", mm, mm_suffix)).as_str());

    return titled_segment("Uptime", uptime);
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

    return titled_segment("Packages", packages);
}

pub fn shell() -> ColoredString {
    let shell = env::var("SHELL").unwrap();

    return titled_segment("Shell", shell);
}

pub fn resolution() -> ColoredString {
    let display = CGDisplay::new(unsafe { CGMainDisplayID() });

    let width = display.pixels_wide();
    let height = display.pixels_high();

    let resolution = format!("{}x{}", width, height);

    return titled_segment("Resolution", resolution);
}

pub fn de() -> ColoredString {
    let de = "Aqua".to_string();

    return titled_segment("DE", de);
}

pub fn wm() -> ColoredString {
    let wm = "Quartz Compositor".to_string();

    return titled_segment("WM", wm);
}

pub fn terminal() -> ColoredString {
    let terminal = env::var("TERM_PROGRAM").unwrap();

    return titled_segment("Terminal", terminal);
}

pub fn cpu() -> ColoredString {
    let cpu = ctl::get_ctl_info("machdep.cpu.brand_string")
        .value_string()
        .unwrap();

    return titled_segment("CPU", cpu);
}

pub fn gpu() -> ColoredString {
    let display = unsafe { CGDisplay::new(CGMainDisplayID()) };
    let mtl_device = unsafe { CGDirectDisplayCopyCurrentMetalDevice(display.id) };
    let gpu = unsafe { mtl_device.get_name().to_string() };

    return titled_segment("GPU", gpu);
}

pub fn battery() -> ColoredString {
    let manager = Manager::new().unwrap();

    let mut batteries = manager.batteries().unwrap();

    let main_battery = batteries.next().unwrap().unwrap();
    let main_battery_charge = main_battery.state_of_charge().value;
    let main_battery_percentage = main_battery_charge * 100.0;

    let battery = String::from(format!("{}%", main_battery_percentage.round()));

    return titled_segment("Battery", battery);
}

pub fn memory() -> ColoredString {
    let system_memory_info = system_info::mem::SystemMemory::new();

    let total = system_memory_info.total / 1024 / 1024;
    let used = system_memory_info.avail / 1024 / 1024;

    let memory = format!("{}MiB / {}MiB", used, total);

    return titled_segment("Memory", memory);
}

pub fn dark_colors() -> ColoredString {
    let mut blocks = "".to_string();

    for i in 40..=47 {
        blocks.push_str(format!("\x1b[{}m   ", i).as_str());
    }

    blocks.push_str("\x1b[0m");

    return blocks.normal();
}

pub fn light_colors() -> ColoredString {
    let mut blocks = "".to_string();

    for i in 100..=107 {
        blocks.push_str(format!("\x1b[{}m{}", i, constants::COLOR_BLOCK_CHARACTER).as_str());
    }

    blocks.push_str("\x1b[0m");

    return blocks.normal();
}

pub fn empty() -> ColoredString {
    return "".normal();
}

fn titled_segment(name: &str, out: String) -> ColoredString {
    return format!("{}: {}", format!("{}", name).bold().yellow(), out).normal();
}
