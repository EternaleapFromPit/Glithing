cmake_minimum_required(VERSION 3.15)
project(System_XUnit LANGUAGES C)

add_library(xunit_runner STATIC native/xunit_runner.c)

target_include_directories(xunit_runner PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/native)
