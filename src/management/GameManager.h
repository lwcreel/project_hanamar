#ifndef GAME_MANAGER_H
#define GAME_MANAGER_H

#include "../map/GameMap.h"
#include <chrono>

class GameManager {

  std::chrono::steady_clock main_clock;

public:
  GameManager();
};

#endif
