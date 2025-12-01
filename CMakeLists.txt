cmake_minimum_required(VERSION 3.16)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/")

option(Year "Which year to build" OFF)
option(Day "Which day to build" OFF)
option(Part "Which part to build" OFF)

if (${Year} AND ${Day} AND ${Part})
    project(aoc
        LANGUAGES CXX
    )

    set(SOURCES
        ${PROJECT_SOURCE_DIR}/${Year}/${Day}/${Part}.cpp
    )

    add_compile_options(-Wall -Werror -Wpedantic)

    add_executable(${PROJECT_NAME} ${SOURCES})

    add_custom_command(
        TARGET aoc POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
                ${PROJECT_SOURCE_DIR}/${Year}/${Day}/input.txt
                ${CMAKE_CURRENT_BINARY_DIR}/input.txt)
endif()
