#include "GameMap.h"
#include <iostream>

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

void GameMap::renderEntityAtPos(Entity *entity) {
  this->mapGrid.at(entity->x).at(entity->y) = entity->entityIcon;
}

void GameMap::renderAtPosition(int x, int y, std::string s) {
  this->mapGrid.at(x).at(y) = s;
}
