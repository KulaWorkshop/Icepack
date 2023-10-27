cmake_minimum_required(VERSION 3.10)

project(lzrw)

add_library(lzrw STATIC lzrw.c lzrw3.c lzrw3-a.c)

target_include_directories(lzrw PRIVATE .)

install(TARGETS lzrw DESTINATION .)