#ifndef UTILS_HPP
#define UTILS_HPP

#include <string>
#include <vector>

#include "./segments.hpp"

using std::string;
using std::vector;

string exec(const char* cmd);

void cache(string name, string value);
void display(string username, string hostname, vector<string> ascii, vector<segment> segments);

#endif
