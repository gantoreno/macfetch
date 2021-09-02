#include <functional>
#include <iostream>
#include <vector>

#include "../include/ascii.hpp"
#include "../include/utils.hpp"
#include "../include/segments.hpp"
#include "../include/descriptors.hpp"

using std::cout;
using std::endl;

int main(int argc, char** argv)
{
    if (argc > 1)
    {
        if (strcmp(argv[1], "--recache") == 0)
        {
            exec("rm -rf /Library/Caches/macfetch");
        }
        else if (strcmp(argv[1], "--help") == 0)
        {
            cout << "Usage: macfetch [--recache | --help]" << endl;

            return 1;
        }
    }

    if (strcmp(exec("uname").c_str(), "Darwin") != 0)
    {
        cout << "Error: OS unsupported" << endl;

        return 1;
    }

    string username = exec("whoami");
    string hostname = exec("hostname");

    vector<string> ascii = ascii_startrail;
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
        segment("Battery", battery),
        segment(empty),
        segment(dark_colors),
        segment(bright_colors),
    };

    display(username, hostname, ascii, segments);

    return 0;
}
