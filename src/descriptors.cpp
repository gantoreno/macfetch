#include <iostream>
#include <vector>
#include <string>

#include "../include/colors.hpp"
#include "../include/utils.hpp"
#include "../include/descriptors.hpp"

using std::vector;
using std::string;
using std::to_string;

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

    out = exec("ls /opt/homebrew/Cellar/* 2> /dev/null | grep ':' | wc -l | xargs || echo '0'") + " (brew)";

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

void* wm(string& out)
{
     string info = exec("ps -e | grep -o \
                    -e \"[S]pectacle\" \
                    -e \"[A]methyst\" \
                    -e \"[k]wm\" \
                    -e \"[c]hun[k]wm\" \
                    -e \"[y]abai\" \
                    -e \"[R]ectangle\"");

    if (info.empty())
    {
        info = "Quartz Compositor";
    }
    else
    {
        info = info.substr(0, info.length() / 2);
    }

    set_cache("info", info);

    out = info;
    
    return NULL;
}

void* terminal(string& out)
{
    out = exec("echo $TERM_PROGRAM");

    return NULL;
}

void* cpu(string& out)
{
    string cached = get_cache("cpu");

    if (cached.empty())
    {
        string info = exec("sysctl -n machdep.cpu.brand_string");

        set_cache("cpu", info);

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
    string cached = get_cache("gpu");

    if (cached.empty())
    {
        string info = exec("system_profiler SPDisplaysDataType 2> /dev/null | awk -F': ' '/^\\ *Chipset Model:/ {printf $2}'");

        set_cache("gpu", info);

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
    int total_memory = stoi(exec("sysctl -n hw.pagesize"));

    float memory_free_percentage = stof(exec("memory_pressure | grep 'percentage' | awk '{print $5}' | sed -r 's/[^0-9]*//g' || echo 0"));
    float memory_used_percentage = 100 - memory_free_percentage;

    int used_memory = total_memory * (memory_used_percentage / 100);

    out = to_string(used_memory) + "MiB / " + to_string(total_memory) + "MiB";

    return NULL;
}

void* battery(string& out)
{
    out = exec("pmset -g batt | grep -Eo \"\\d+%\"");

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
