#ifndef UTILS_H
#define UTILS_H

#include <string>
#include <vector>

#include "./segments.h"

using std::string;
using std::vector;

string exec(const char* cmd);

void cache(string name, string value);
void display(string username, string hostname, vector<string> ascii, vector<segment> segments);

#endif
