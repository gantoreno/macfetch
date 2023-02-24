pub mod constants;

use colored::{ColoredString, Colorize};

pub fn generate_logo(name: &str) -> Vec<ColoredString> {
    return match name {
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
        constants::DARWIN | _ => vec![
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
    };
}
