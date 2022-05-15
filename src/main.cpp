#include <functional>
#include <iostream>
#include <vector>

#include "../include/ascii.hpp"
#include "../include/utils.hpp"
#include "../include/segments.hpp"
#include "../include/constants.hpp"
#include "../include/descriptors.hpp"

using std::cout;
using std::endl;

void help()
{
    cout << "Usage: macfetch [options]" << "\n"
            << "    -h, --help           To echo this help message" << "\n"
            << "    -v, --version        To see the version number" << "\n"
            << "    -r, --recache        To remove all cached data" << "\n";
}

int main(int argc, char** argv)
{
    if (argc > 1)
    {
        if (strcmp(argv[1], "-r") == 0 || strcmp(argv[1], "--recache") == 0)
        {
            exec("rm -rf /Library/Caches/macfetch");
        }
        else if (strcmp(argv[1], "-v") == 0 || strcmp(argv[1], "--version") == 0)
        {
            cout << "Macfetch v" VERSION << endl;

            return 0;
        }
        else if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)
        {
            help();

            return 0;
        }
    }

    if (strcmp(exec("uname").c_str(), "Darwin") != 0)
    {
        cout << "Error: OS unsupported" << endl;

        return 1;
    }

    string username = exec("whoami");
    string hostname = exec("hostname");

    vector<string> ascii = ascii_darwin;
    vector<segment> segments = {
        segment("OS", os),
        segment("Host", host),
        segment("Kernel", kernel),
        segment("Uptime", uptime),
        segment("Packages", packages),
        segment("Shell", shell),
        segment("Resolution", resolution),
        segment("DE", de),
        segment("Terminal", terminal),
        segment("CPU", cpu),
        segment("GPU", gpu),
        segment("Memory", memory),
        segment(empty),
        segment(dark_colors),
        segment(bright_colors),
    };

    display(username, hostname, ascii, segments);

    return 0;
}
