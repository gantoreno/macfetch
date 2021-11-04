#ifndef UTILS_HPP
#define UTILS_HPP

#include <vector>
#include <string>

#include "./segments.hpp"

using std::vector;
using std::string;

string exec(const char* cmd);

void cache(string name, string value);
void display(string username, string hostname, vector<string> ascii, vector<segment> segments);

#endif
