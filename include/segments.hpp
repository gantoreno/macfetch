#ifndef SEGMENTS_HPP
#define SEGMENTS_HPP

#include <string>

using std::function;
using std::string;

struct segment
{
    string name;
    function<void*(string&)> descriptor;

    segment();
    segment(function<void*(string&)> descriptor);
    segment(string name, function<void*(string&)> descriptor);
};

#endif
