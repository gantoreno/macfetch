mod macfetch;

use crate::macfetch::ascii;
use crate::macfetch::utils::config;

fn main() {
    macfetch::utils::cli::handle_cli_args();

    let segments = config::get_segments();

    macfetch::render(ascii::constants::DARWIN, segments);
}
