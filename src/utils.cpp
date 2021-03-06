#include <iostream>
#include <thread>
#include <vector>
#include <string>
#include <regex>
#include <array>

#include "../include/constants.hpp"
#include "../include/colors.hpp"
#include "../include/utils.hpp"

using std::array;
using std::cout;
using std::ref;
using std::string;
using std::thread;
using std::unique_ptr;
using std::vector;

string exec(const char* cmd)
{
    string result;
    array<char, 128> buffer;
    unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);

    if (!pipe)
    {
        throw std::runtime_error("popen() failed!");
    }

    while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr)
    {
        result += regex_replace(buffer.data(), std::regex("\n"), "");
    }

    return result;
}

void cache(string name, string value)
{
    exec("mkdir -p /Library/Caches/macfetch");
    exec(("echo '" + value + "' > /Library/Caches/macfetch/" + name).c_str());
}

void display(string username, string hostname, vector<string> ascii, vector<segment> segments)
{
    int ascii_size = ascii.size();
    int segments_size = segments.size() + RESERVED_LINES;

    int rows = ascii_size > segments_size ? ascii_size : segments_size;

    thread segment_threads[segments_size - RESERVED_LINES];
    string segment_outputs[segments_size - RESERVED_LINES];

    for (int i = 0; i < segments_size - RESERVED_LINES; i++)
    {
        segment_threads[i] = thread(segments.at(i).descriptor, ref(segment_outputs[i]));
    }

    for (int i = 0; i < rows; i++)
    {
        string line = "";

        line += i < ascii_size ? ascii.at(i) : ascii.at(ascii_size - 1);
        line += "   ";

        switch (i - RESERVED_LINES)
        {
        case -2:
            line += BOLD FG2 + username + DEFAULT "@" BOLD FG2 + hostname + DEFAULT;
            break;
        case -1:
            line += string(username.length() + hostname.length() + 1, '-');
            break;
        default:
            if (i < segments_size)
            {
                segment_threads[i - RESERVED_LINES].join();

                string name = segments.at(i - RESERVED_LINES).name;
                string output = segment_outputs[i - RESERVED_LINES];

                line += (!name.empty() ? BOLD FG3 + name + DEFAULT + ": " : "") + output;
            }
            break;
        }

        cout << line << "\n";
    }

    cout << (rows == segments_size ? "\n" : "");
}
