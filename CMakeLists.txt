cmake_minimum_required(VERSION 3.15...3.31)

project(project_hanamar VERSION 1.0
	DESCRIPTION "Simple C++ ASCII Game"
	LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE SOURCES "src/*.cpp" "src/entities/*.cpp" "src/entities/mobs/*.cpp" "src/entities/player/*.cpp" "src/management/*.cpp" "src/map/*.cpp" "src/physics/*.cpp" "src/utils/*.cpp")

add_executable(myapp ${SOURCES})   

target_include_directories(myapp PRIVATE src)   
