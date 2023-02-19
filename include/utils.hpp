#ifndef UTILS_HPP
#define UTILS_HPP

#include <vector>
#include <string>

#include "./segments.hpp"

using std::vector;
using std::string;

string exec(const char* cmd);

string get_cache(string name);
string set_cache(string name, string value);

void clear_cache();

void display(string username, string hostname, vector<string> ascii, vector<segment> segments);

#endif
