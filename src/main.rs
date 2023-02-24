mod macfetch;

use crate::macfetch::ascii;
use crate::macfetch::segments::{
    battery, cpu, dark_colors, de, empty, gpu, host, kernel, light_colors, machine, memory, os,
    packages, resolution, separator, shell, terminal, uptime, wm,
};

fn main() {
    macfetch::utils::cli::handle_cli_args();

    macfetch::render(
        ascii::constants::DARWIN,
        vec![
            machine,
            separator,
            os,
            host,
            kernel,
            uptime,
            packages,
            shell,
            resolution,
            de,
            wm,
            terminal,
            cpu,
            gpu,
            battery,
            memory,
            empty,
            dark_colors,
            light_colors,
        ],
    );
}
