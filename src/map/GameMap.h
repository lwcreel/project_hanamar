#ifndef GAME_MAP_H
#define GAME_MAP_H

#include "../entities/Entity.h"
#include <string>
#include <vector>

// TODO: Convert mapGrid to 1D for better performance?
class GameMap {

  int mapWidth;
  int mapHeight;
  std::vector<std::vector<std::string>> mapGrid;

  bool isOutOfBounds(int x, int y);

public:
  GameMap(int mapWidth, int mapHeight);
  void printMap();
  void writeEntityPosition(Entity *entity);
};

#endif
