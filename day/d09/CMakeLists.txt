cmake_minimum_required(VERSION 3.22)
project(d9 C)

set(CMAKE_C_STANDARD 99)
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/input.txt
        ${CMAKE_CURRENT_BINARY_DIR} COPYONLY)
add_executable(d9p1 part1.c)
add_executable(d9p2 part2.c)