cmake_minimum_required(VERSION 3.5)
project(FruitClassifier VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard and Qt auto features
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)

# Find Qt6 components
find_package(Qt6 REQUIRED COMPONENTS Widgets)

# Set project structure variables
set(UI_DIR ${CMAKE_CURRENT_SOURCE_DIR}/ui)
set(SRC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/src)
set(INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

# UI files
file(GLOB UI_FILES 
    ${UI_DIR}/*.ui
)

# Source and header files
set(SOURCE_FILES
    ${SRC_DIR}/main.cpp
    ${SRC_DIR}/mainwindow.cpp
    ${SRC_DIR}/plotwidget.cpp
)

set(HEADER_FILES
    ${INCLUDE_DIR}/mainwindow.h
    ${INCLUDE_DIR}/plotwidget.h
)

# Include directories
include_directories(
    ${INCLUDE_DIR}
    ${SRC_DIR}
)

# Create executable
add_executable(FruitClassifier
    ${SOURCE_FILES}
    ${HEADER_FILES}
    ${UI_FILES}
)

# Platform-specific Rust library extension
if(WIN32)
    set(RUST_LIB_EXT ".dll")
    set(RUST_LIB_PREFIX "")
elseif(APPLE)
    set(RUST_LIB_EXT ".dylib")
    set(RUST_LIB_PREFIX "lib")
else()
    set(RUST_LIB_EXT ".so")
    set(RUST_LIB_PREFIX "lib")
endif()

# Find and link Rust library
set(RUST_LIB_NAME "${RUST_LIB_PREFIX}fruit_classifier${RUST_LIB_EXT}")
set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../rust_backend/target/release")

find_library(FRUIT_CLASSIFIER_LIB
    NAMES ${RUST_LIB_NAME}
    PATHS ${RUST_LIB_DIR}
    NO_DEFAULT_PATH
    REQUIRED
)

message(STATUS "Found Rust library: ${FRUIT_CLASSIFIER_LIB}")

target_link_libraries(FruitClassifier PRIVATE 
    Qt6::Widgets
    ${FRUIT_CLASSIFIER_LIB}
)

# Copy Rust library to build directory
add_custom_command(TARGET FruitClassifier POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${FRUIT_CLASSIFIER_LIB}
    $<TARGET_FILE_DIR:FruitClassifier>
)

# Installation (optional)
install(TARGETS FruitClassifier
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

# Add debug/release configurations
set(CMAKE_BUILD_TYPE RelWithDebInfo CACHE STRING "Choose the type of build" FORCE)
set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS "Debug" "Release" "MinSizeRel" "RelWithDebInfo")