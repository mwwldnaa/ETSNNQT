cmake_minimum_required(VERSION 3.16)
project(FruitClassifier)

set(CMAKE_AUTOMOC ON)         # <== INI WAJIB
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Qt packages
find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets)

# Set source and include directories
set(SOURCE_DIR ${CMAKE_SOURCE_DIR}/src)
set(INCLUDE_DIR ${CMAKE_SOURCE_DIR}/src)

# Set Rust library path
set(RUST_LIBRARY_DIR "${CMAKE_SOURCE_DIR}/../rust_backend/target/release")
set(RUST_LIBRARY_NAME "fruit_classifier")
set(RUST_LIBRARY_FULL_PATH "${RUST_LIBRARY_DIR}/lib${RUST_LIBRARY_NAME}.so")

if(NOT EXISTS ${RUST_LIBRARY_FULL_PATH})
    message(WARNING "Rust library not found at: ${RUST_LIBRARY_FULL_PATH}")
else()
    message(STATUS "Found Rust library at: ${RUST_LIBRARY_FULL_PATH}")
endif()

add_library(rust_library SHARED IMPORTED)
set_target_properties(rust_library PROPERTIES
    IMPORTED_LOCATION ${RUST_LIBRARY_FULL_PATH}
)

# Add executable with source files
add_executable(FruitClassifier
    src/main.cpp
    src/mainwindow.cpp
    src/mainwindow.h
    src/plotwidget.cpp
    src/plotwidget.h
    ui/mainwindow.ui
)

# Include directories
target_include_directories(FruitClassifier PRIVATE
    ${INCLUDE_DIR}
)

# Link libraries
target_link_libraries(FruitClassifier
    Qt${QT_VERSION_MAJOR}::Widgets
    rust_library
)

if(QT_VERSION_MAJOR EQUAL 6)
    qt_add_executable(FruitClassifier MANUAL_FINALIZATION)
    qt_finalize_executable(FruitClassifier)
endif()