cmake_minimum_required(VERSION 3.5)
project(day12swift Swift)
add_library(day12swift SHARED Sources/Day12/Day12.swift)
install(TARGETS day12swift DESTINATION .)
