#include <iostream>
#include <thread>
#include <string>
#include <vector>
#include <regex>
#include <array>

#include "../include/colors.hpp"
#include "../include/utils.hpp"

using std::array;
using std::cout;
using std::endl;
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
    int segments_size = segments.size() + 2;

    int rows = ascii_size > segments_size ? ascii_size : segments_size;

    thread segment_threads[segments_size - 2];
    string segment_outputs[segments_size - 2];

    for (int i = 0; i < segments_size - 2; i++)
    {
        segment_threads[i] = thread(segments.at(i).descriptor, ref(segment_outputs[i]));
    }

    for (int i = 0; i < rows; i++)
    {
        string line = "";

        line += i < ascii_size ? ascii.at(i) : ascii.at(ascii_size - 1);
        line += "   ";

        switch (i - 2)
        {
        case -2:
            line += BOLD FG12 + username + DEFAULT "@" BOLD FG12 + hostname + DEFAULT;
            break;
        case -1:
            line += string(username.length() + hostname.length() + 1, '-');
            break;
        default:
            if (i < segments_size)
            {
                segment_threads[i - 2].join();

                string name = segments.at(i - 2).name;
                string output = segment_outputs[i - 2];

                line += (!name.empty() ? BOLD FG4 + name + DEFAULT + ": " : "") + output;
            }
            break;
        }

        cout << line << endl;
    }

    cout << (rows == segments_size ? "\n" : "");
}
