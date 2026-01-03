pub mod constants;

use colored::{ColoredString, Colorize};

pub fn generate_logo(name: &str) -> Vec<ColoredString> {
    match name {
        constants::GABRIEL => vec![
            "            .xWXc ".bold(),
            "            :XMMx.".bold(),
            "    .;lodddkXMMX: ".bold(),
            "  'dKWMMMMMMMMXc  ".bold(),
            " :KMMNkWMMWXWMWx. ".bold(),
            "'0MMKK'    'KWMWo ".bold(),
            ":XMMx:      :XMMx.".bold(),
            "'0MMXc.    .xWMNl ".bold(),
            " ;KMMNOollxKWMNd. ".bold(),
            "  .o0NMMMMMMMWx.  ".bold(),
            "    .,clodkXMMNo. ".bold(),
            " .';.      'kWMNl ".bold(),
            ",KMNl       :XMMx.".bold(),
            ",KMM0;     .dWMWo ".bold(),
            " cXMMXxcmxoOWMWk. ".bold(),
            "  ,xXWMMMMMMW0l.  ".bold(),
            "   .;xXWMWN0l.    ".bold(),
        ],
        _ => vec![
            "                    'c.       ".bold().green(),
            "                 ,xNMM.       ".bold().green(),
            "               .OMMMMo        ".bold().green(),
            "               OMMM0,         ".bold().green(),
            "     .;loddo:' loolloddol;.   ".bold().green(),
            "   cKMMMMMMMMMMNWMMMMMMMMMM0: ".bold().green(),
            " .KMMMMMMMMMMMMMMMMMMMMMMMWd. ".bold().yellow(),
            " XMMMMMMMMMMMMMMMMMMMMMMMX.   ".bold().yellow(),
            ";MMMMMMMMMMMMMMMMMMMMMMMM:    ".bold().red(),
            ":MMMMMMMMMMMMMMMMMMMMMMMM:    ".bold().red(),
            ".MMMMMMMMMMMMMMMMMMMMMMMMX.   ".bold().red(),
            " kMMMMMMMMMMMMMMMMMMMMMMMMWd. ".bold().red(),
            ".XMMMMMMMMMMMMMMMMMMMMMMMMMMk ".bold().blue(),
            "  .XMMMMMMMMMMMMMMMMMMMMMMMMK.".bold().blue(),
            "    kMMMMMMMMMMMMMMMMMMMMMMd  ".bold().magenta(),
            "     ;KMMMMMMMWXXWMMMMMMMk.   ".bold().magenta(),
            "       .cooc,.    .,coo:.     ".bold().magenta(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_logo_darwin_returns_correct_lines() {
        let logo = generate_logo(constants::DARWIN);
        assert_eq!(logo.len(), 17);
    }

    #[test]
    fn test_generate_logo_gabriel_returns_correct_lines() {
        let logo = generate_logo(constants::GABRIEL);
        assert_eq!(logo.len(), 17);
    }

    #[test]
    fn test_generate_logo_unknown_defaults_to_darwin() {
        let logo = generate_logo("unknown");
        assert_eq!(logo.len(), 17);
    }
}
