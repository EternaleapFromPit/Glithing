cmake_minimum_required(VERSION 3.15)
project(System_XUnit LANGUAGES C)

# The xUnit runner is now provided by the Rust compiler runtime.
# Keep this target as an interface placeholder for package metadata.
add_library(xunit_runner INTERFACE)
target_include_directories(xunit_runner INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/native)
