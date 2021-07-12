#include <iostream>
#include <vector>
#include <string>

#include "../include/colors.hpp"
#include "../include/utils.hpp"
#include "../include/segments.hpp"
#include "../include/descriptors.hpp"

using std::string;

segment::segment() :
    name(""), descriptor(empty){};

segment::segment(function<void*(string&)> descriptor) :
    name(""), descriptor(descriptor){};

segment::segment(string name, function<void*(string&)> descriptor) :
    name(name), descriptor(descriptor){};
