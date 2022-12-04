cmake_minimum_required(VERSION 3.23)
project(d4 C)

set(CMAKE_C_STANDARD 99)
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/input.txt
        ${CMAKE_CURRENT_BINARY_DIR} COPYONLY)

add_executable(d4p1 part1.c)
add_executable(d4p2 part2.c)
