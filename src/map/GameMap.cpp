#include "GameMap.h"
#include <iostream>

bool GameMap::isOutOfBounds(int x, int y) {
  return !(x < this->mapWidth || x >= 0 || y < this->mapHeight || y >= 0);
}

GameMap::GameMap(int mapWidth, int mapHeight) {
  this->mapWidth = mapWidth;
  this->mapHeight = mapHeight;
  this->mapGrid = std::vector<std::vector<std::string>>(
      mapWidth, std::vector<std::string>(mapHeight, " "));
}

void GameMap::printMap() {

  for (int x = 0; x < this->mapWidth; x++) {
    for (int y = 0; y < this->mapHeight; y++) {
      std::cout << this->mapGrid[x][y];
    }
    std::cout << std::endl;
  }
}

void GameMap::writeEntityPosition(Entity *entity) {

  this->mapGrid[entity->y][entity->x] = entity->entityIcon;
}
