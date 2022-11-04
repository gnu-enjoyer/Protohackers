cmake_minimum_required(VERSION 3.16)

project(echo-server)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_EXE_LINKER_FLAGS "-static")

set(CMAKE_THREAD_PREFER_PTHREAD TRUE)

find_package(Threads REQUIRED)

if(CMAKE_USE_PTHREADS_INIT)
    set(CMAKE_CXX_FLAGS ${CMAKE_CXX_FLAGS} "-pthread")
endif()

add_executable(echo-server
        ${CMAKE_CURRENT_SOURCE_DIR}/src/server.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp)

target_link_libraries(echo-server
        ${CMAKE_THREAD_LIBS_INIT})
        