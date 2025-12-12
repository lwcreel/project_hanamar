cmake_minimum_required(VERSION 3.15...3.31)

project(project_hanamar VERSION 1.0
	DESCRIPTION "Simple C++ ASCII Game"
	LANGUAGES CXX)

add_executable(main src/main.cpp src/entities/player/Player.h src/entities/mobs/Mob.h src/entities/Entity.cpp src/map/GameMap.cpp)
