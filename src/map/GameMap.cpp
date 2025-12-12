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

void GameMap::PrintEntity(int entityPosX, int entityPosY, char entityIcon) {}

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

void GameMap::PrintMobPosition(int mob_position_x, int mob_position_y,
                               std::string mob_icon) {

  if (OutOfBounds(mob_position_x, mob_position_y)) {
    std::cout << "ERROR: Out of Bounds!" << std::endl;
    return;
  }

  std::string curr_user_row;
  curr_user_row.append("|");

  for (int curr_x = 0; curr_x < mapBoundaryX - 1; curr_x++) {
    if (curr_x != mob_position_x)
      curr_user_row.append(" ");
    else
      curr_user_row.append(mob_icon);
  }

  curr_user_row.append("|");

  std::cout << mapBorderX << std::endl;

  for (int curr_y = 0; curr_y < mapBoundaryY; curr_y++) {
    if (mob_position_y != curr_y) {
      std::cout << mapRow << std::endl;
      continue;
    }
    std::cout << curr_user_row << std::endl;
  }

  std::cout << mapBorderX << std::endl;
  return;
}
