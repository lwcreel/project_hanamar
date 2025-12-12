#include "GameMap.h"
#include <iostream>

bool GameMap::OutOfBounds(int x_pos, int y_pos) {
  return !(x_pos < mapBoundaryX || x_pos >= 0 || y_pos < mapBoundaryY ||
           y_pos >= 0);
}

GameMap::GameMap(int mapWidth, int mapHeight) {

  this->mapBoundaryX = mapWidth;
  this->mapBoundaryY = mapHeight;

  for (int x = 0; x < this->mapBoundaryX; x++) {
    this->mapBorderX.append("-");
  }
  this->mapBorderX.append("-");

  this->mapRow.append("|");

  for (int x = 1; x < this->mapBoundaryX; x++) {
    this->mapRow.append(" ");
  }
  this->mapRow.append("|");

  for (int i = 0; i < mapWidth; i++) {
    for (int j = 0; j < mapHeight; j++) {

      this->mapGrid[i][j] = '-';
    }
  }
}

void GameMap::PrintMap() {

  std::cout << mapBorderX << std::endl;

  for (int y = 0; y < mapBoundaryY; y++) {
    std::cout << mapRow << std::endl;
  }

  std::cout << mapBorderX << std::endl;
  return;
}

void GameMap::PrintEntity(Entity *entity) {

  this->mapGrid[entity->y][entity->x] = entity->entityIcon;
}

void GameMap::PrintUserPosition(int userPosX, int userPosY) {

  if (OutOfBounds(userPosX, userPosY)) {
    std::cout << "ERROR: Out of Bounds!" << std::endl;
    return;
  }

  std::string userRow;
  userRow.append("|");

  for (int x = 0; x < mapBoundaryX - 1; x++) {
    if (x != userPosX)
      userRow.append(" ");
    else
      userRow.append("@");
  }

  userRow.append("|");

  std::cout << mapBorderX << std::endl;

  for (int y = 0; y < mapBoundaryY; y++) {
    if (userPosY != y) {
      std::cout << mapRow << std::endl;
      continue;
    }
    std::cout << userRow << std::endl;
  }

  std::cout << mapBorderX << std::endl;
  return;
}
