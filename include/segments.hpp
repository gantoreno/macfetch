#ifndef SEGMENTS_HPP
#define SEGMENTS_HPP

#include <string>

using std::function;
using std::string;

struct segment
{
    string name;
    function<void*(string&)> descriptor;

    segment(string name, function<void*(string&)> descriptor);
    segment(function<void*(string&)> descriptor);
};

void* os(string& out);
void* host(string& out);
void* kernel(string& out);
void* uptime(string& out);
void* packages(string& out);
void* shell(string& out);
void* resolution(string& out);
void* de(string& out);
void* terminal(string& out);
void* cpu(string& out);
void* gpu(string& out);
void* memory(string& out);
void* empty(string& out);
void* dark_colors(string& out);
void* bright_colors(string& out);

#endif
