use std::{env, process::exit};

use crate::macfetch::constants;

pub fn handle_cli_args() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let flag = args[1].as_str();

        match flag {
            "--help" | "-h" => print_help_message(),
            "--version" | "-v" => print_version(),
            _ => print_unkown_flag(flag),
        }
    }
}

pub fn print_version() {
    println!("Macfetch v{}", constants::VERSION);

    exit(0);
}

pub fn print_help_message() {
    println!("Usage: macfetch [options]");
    println!("    -h, --help           To echo this help message");
    println!("    -v, --version        To see the version number");

    exit(0);
}

pub fn print_unkown_flag(flag: &str) {
    println!("Unknown arg: {}", flag);

    exit(1);
}
