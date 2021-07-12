cmake_minimum_required(VERSION 3.0)

project(iris)

set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++14")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "./out")

add_executable(
  macfetch
  src/main.cpp
  src/ascii.cpp
  src/descriptors.cpp
  src/utils.cpp
  src/segments.cpp
)
