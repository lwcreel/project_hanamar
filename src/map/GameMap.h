#ifndef GAME_MAP_H
#define GAME_MAP_H

#include "../entities/Entity.h"
#include <string>
#include <vector>

class GameMap {

  int mapBoundaryX;
  int mapBoundaryY;
  std::string mapBorderX;
  std::string mapRow;
  std::vector<std::vector<std::string>> mapGrid;

  bool OutOfBounds(int x, int y);

public:
  GameMap(int mapWidth, int mapHeight);
  void PrintMap();
  void PrintEntity(Entity *entity);
  void PrintUserPosition(int user_position_x, int user_position_y);
  void PrintMobPosition(int mob_position_x, int mob_position_y,
                        std::string mob_icon);
};

#endif
