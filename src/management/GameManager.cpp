#include "GameManager.h"

GameManager::GameManager(int mapWidth, int mapHeight)
    : gameMap(GameMap(mapWidth, mapHeight)), physicsEngine() {

  std::unordered_map<std::string, std::list<Entity>> entities = {
      {"player", {Player()}}, {"mobs", {Mob()}}};
}

void GameManager::spawnEntity(Entity *entity) {

  this->gameMap.writeEntityPosition(entity);
}

void GameManager::updateEntityPosition(Entity *entity) {}
