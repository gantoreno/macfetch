#include <vector>
#include <string>

#include "../include/ascii.hpp"
#include "../include/colors.hpp"

using std::string;
using std::vector;

vector<string> ascii_darwin = {
    BOLD FG2 "                    'c.       " DEFAULT,
    BOLD FG2 "                 ,xNMM.       " DEFAULT,
    BOLD FG2 "               .OMMMMo        " DEFAULT,
    BOLD FG2 "               OMMM0,         " DEFAULT,
    BOLD FG2 "     .;loddo:' loolloddol;.   " DEFAULT,
    BOLD FG2 "   cKMMMMMMMMMMNWMMMMMMMMMM0: " DEFAULT,
    BOLD FG3 " .KMMMMMMMMMMMMMMMMMMMMMMMWd. " DEFAULT,
    BOLD FG3 " XMMMMMMMMMMMMMMMMMMMMMMMX.   " DEFAULT,
    BOLD FG1 ";MMMMMMMMMMMMMMMMMMMMMMMM:    " DEFAULT,
    BOLD FG1 ":MMMMMMMMMMMMMMMMMMMMMMMM:    " DEFAULT,
    BOLD FG1 ".MMMMMMMMMMMMMMMMMMMMMMMMX.   " DEFAULT,
    BOLD FG1 " kMMMMMMMMMMMMMMMMMMMMMMMMWd. " DEFAULT,
    BOLD FG5 ".XMMMMMMMMMMMMMMMMMMMMMMMMMMk " DEFAULT,
    BOLD FG5 "  .XMMMMMMMMMMMMMMMMMMMMMMMMK." DEFAULT,
    BOLD FG4 "    kMMMMMMMMMMMMMMMMMMMMMMd  " DEFAULT,
    BOLD FG4 "     ;KMMMMMMMWXXWMMMMMMMk.   " DEFAULT,
    BOLD FG4 "       .cooc,.    .,coo:.     " DEFAULT,
    "                              "
};

vector<string> ascii_startrail = {
    BOLD FG12 "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠤⠔⠒⠈⠉⠉⠉⠈⠉⠉⠀⠐⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠀⠀⠀⠀⠀⣠⠔⠚⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠀⠀⢀⡤⠚⠁⠀⠀⠀⠀⠀⢀⣀⡠⠤⠤⠀⠀⠠⠤⢄⣀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⣠⠔⠊⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠐⠢⣀⠀⠀⠀⠀⠈⠱⣆⠀⠀⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠄⠀⠀⠀⠀⠈⢣⡀⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠀⠀⠀⠀⢠⠎⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠀⠀⠀⠀⠀⠳⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠀⠀⠀⣰⠃⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠄⠀⠀⠀⠀⢣⠀" DEFAULT,
    BOLD FG12 "⠰⠀⠀⠀⠀⢰⠃⠀⠀⠀⠀⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡀⠀⠀⠀⠈⡄⠀⠀⠀⠈⡆" DEFAULT,
    BOLD FG12 "⡆⠀⠀⠀⠀⡎⠀⠀⠀⠀⡘⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀⠹⣆⠀⠀⠀⠀⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹" DEFAULT,
    BOLD FG12 "⠃⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⢸⡄⠀⠀⠀⢸⠀⠀⠀⠀⢰⠀⠀⠀⠀⢸" DEFAULT,
    BOLD FG12 "⡇⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⣸⠁⠀⠀⠀⠸⠀⠀⠀⠀⠈⠀⠀⠀⠀⢸" DEFAULT,
    BOLD FG12 "⢇⠀⠀⠀⠀⢣⠀⠀⠀⠀⢹⠀⠀⠀⠀⠈⠂⡀⠀⠀⠀⠀⢀⡔⠁⠀⠀⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈" DEFAULT,
    BOLD FG12 "⠸⡀⠀⠀⠀⠘⣆⠀⠀⠀⠀⢳⡀⠀⠀⠀⠀⠈⠀⠐⠂⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠁⠀⠀⠀⢀⠁" DEFAULT,
    BOLD FG12 "⠀⠱⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠙⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠆⠀" DEFAULT,
    BOLD FG12 "⠀⠀⢣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠳⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀" DEFAULT,
    BOLD FG12 "⠀⠀⠀⠀⠉⢢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀" DEFAULT,
    "⠀⠀⠀⠀   ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀"
};
