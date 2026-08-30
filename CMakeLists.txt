cmake_minimum_required(VERSION 3.21)
project(LarTycc VERSION 0.0.1 LANGUAGES CXX)

option(LARTYCC_BUILD_TESTS "Build LarTycc C++ tests" ON)
if(LARTYCC_BUILD_TESTS)
  include(CTest)
endif()
add_subdirectory(audio-engine)
