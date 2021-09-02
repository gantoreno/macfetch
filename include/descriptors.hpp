#ifndef DESCRIPTORS_HPP
#define DESCRIPTORS_HPP

#include <string>

using std::string;

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
void* battery(string& out);
void* empty(string& out);
void* dark_colors(string& out);
void* bright_colors(string& out);

#endif
