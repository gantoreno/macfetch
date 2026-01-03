pub mod ascii;
pub mod constants;
pub mod segments;
pub mod utils;

use colored::{ColoredString, Colorize};

pub fn render(logo: &str, segments: Vec<fn() -> ColoredString>) {
    let logo = ascii::generate_logo(logo);

    let length = if logo.len() > segments.len() {
        logo.len()
    } else {
        segments.len()
    } + constants::RESERVED_LINES;

    for i in 0..length {
        let logo_line = match logo.get(i) {
            Some(line) => line.to_owned(),
            None => String::from(" ")
                .repeat(logo.first().unwrap().len())
                .normal(),
        };
        let segment_line = match segments.get(i) {
            Some(function) => function().to_owned(),
            None => "".normal(),
        };

        println!("{}    {}", logo_line, segment_line);
    }
}
