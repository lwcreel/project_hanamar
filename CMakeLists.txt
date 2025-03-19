cmake_minimum_required(VERSION 3.15...3.31)

project(project_hanamar VERSION 1.0
	DESCRIPTION "Simple C++ ASCII Game"
	LANGUAGES CXX)

add_executable(main src/main.cpp src/entities/player/player.h src/entities/mobs/mob.h src/entities/entity.cpp src/map/game_map.cpp)
