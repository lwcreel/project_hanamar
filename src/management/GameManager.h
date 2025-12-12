#ifndef GAME_MANAGER_H
#define GAME_MANAGER_H

#include "../entities/Entity.h"
#include "../entities/mobs/Mob.h"
#include "../entities/player/Player.h"
#include "../map/GameMap.h"
#include "../physics/PhysicsEngine.h"
#include <chrono>
#include <list>
#include <string>
#include <unordered_map>

class GameManager {

  //   std::chrono::steady_clock gameClock;
  GameMap gameMap;
  PhysicsEngine physicsEngine;

public:
  GameManager(int mapWidth, int mapHeight);
  void spawnEntity(Entity *entity);
  void updateEntityPosition(Entity *entity);
};

#endif
