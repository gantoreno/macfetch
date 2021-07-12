#include <string>
#include <vector>

#include "../include/colors.hpp"
#include "../include/utils.hpp"
#include "../include/descriptors.hpp"

using std::string;
using std::to_string;
using std::vector;

void* os(string& out)
{
    string os_version = exec("sw_vers | xargs | awk '{print $2,$4}'");
    string os_architecture = exec("uname -m");

    out = os_version + " " + os_architecture;

    return NULL;
}

void* host(string& out)
{
    out = exec("sysctl -n hw.model");

    return NULL;
}

void* kernel(string& out)
{
    out = exec("uname -r");

    return NULL;
}

void* uptime(string& out)
{
    int uptimeD;
    int uptimeH;
    int uptimeM;

    struct timespec time;

    clock_gettime(CLOCK_MONOTONIC, &time);

    int dd = time.tv_sec / 60 / 60 / 24;
    int hh = time.tv_sec / 60 / 60 % 24;
    int mm = time.tv_sec / 60 % 60;

    string final_uptime = "";

    final_uptime += to_string(dd) + " day" + (dd == 1 ? "" : "s") + ", ";
    final_uptime += to_string(hh) + " hour" + (hh == 1 ? "" : "s") + ", ";
    final_uptime += to_string(mm) + " minute" + (mm == 1 ? "" : "s");

    out = final_uptime;

    return NULL;
}

void* packages(string& out)
{
    bool has_brew = exec("type brew > /dev/null; echo $?") == "0";

    if (!has_brew)
    {
        out = "Unknown";
    }

    out = exec("ls /usr/local/Cellar/* | grep ':' | wc -l | xargs") + " (brew)";

    return NULL;
}

void* shell(string& out)
{
    out = exec("echo $SHELL");

    return NULL;
}

void* resolution(string& out)
{
    out = exec("screenresolution get 2>&1 | awk '/Display/{printf $6}' | awk -F 'x' '{print $1\"x\"$2}'");

    return NULL;
}

void* de(string& out)
{
    out = "Aqua";

    return NULL;
}

void* terminal(string& out)
{
    out = exec("echo $TERM_PROGRAM");

    return NULL;
}

void* cpu(string& out)
{
    string cached = exec("cat 2> /dev/null /Library/Caches/macfetch/cpu");

    if (cached.empty())
    {
        string info = exec("sysctl -n machdep.cpu.brand_string");

        cache("cpu", info);

        out = info;
    }
    else
    {
        out = cached;
    }

    return NULL;
}

void* gpu(string& out)
{
    string cached = exec("cat 2> /dev/null /Library/Caches/macfetch/gpu");

    if (cached.empty())
    {
        string info = exec("system_profiler SPDisplaysDataType 2> /dev/null | awk -F': ' '/^\\ *Chipset Model:/ {printf $2}'");

        cache("gpu", info);

        out = info;
    }
    else
    {
        out = cached;
    }

    return NULL;
}

void* memory(string& out)
{
    string cached = exec("cat 2> /dev/null /Library/Caches/macfetch/memory");

    if (cached.empty())
    {
        string info = exec("system_profiler SPHardwareDataType /dev/null | grep 'Memory:' | awk '{print $2\"\"$3}'");

        cache("memory", info);

        out = info;
    }
    else
    {
        out = cached;
    }

    return NULL;
}

void* dark_colors(string& out)
{
    string blocks = "";
    vector<string> colors = {
        BG0,
        BG1,
        BG2,
        BG3,
        BG4,
        BG5,
        BG6,
        BG7,
    };

    for (string color : colors)
    {
        blocks += color + "   " + DEFAULT;
    }

    out = blocks;

    return NULL;
}

void* bright_colors(string& out)
{
    string blocks = "";
    vector<string> colors = {
        BG8,
        BG9,
        BG10,
        BG11,
        BG12,
        BG13,
        BG14,
        BG15
    };

    for (string color : colors)
    {
        blocks += color + "   " + DEFAULT;
    }

    out = blocks;

    return NULL;
}

void* empty(string& out)
{
    out = "";

    return NULL;
}
