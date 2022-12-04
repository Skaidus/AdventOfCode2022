cmake_minimum_required(VERSION 3.23)
project(AdventOfCode2022)

set(CMAKE_CXX_STANDARD 14)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/input.txt
        ${CMAKE_CURRENT_BINARY_DIR} COPYONLY)

add_executable(day1p1 src/part1.cpp)
add_executable(day1p2 src/part2.cpp)
