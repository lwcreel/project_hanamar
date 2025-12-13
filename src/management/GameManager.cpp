#include "GameManager.h"

GameManager::GameManager(int mapWidth, int mapHeight)
    : gameMap(GameMap(mapWidth, mapHeight)), physicsEngine() {

  std::unordered_map<std::string, std::list<Entity>> entities = {
      {"player", {Player()}}, {"mobs", {Mob()}}};
}

void GameManager::spawnEntity(Entity *entity) {

  // Update PhysicsEngine
  std::unordered_map<int, Position> entityPositions =
      this->physicsEngine.getEntityPositions();
  entityPositions[entity->getId()] = {entity->getX(), entity->getY()};
  this->physicsEngine.setEntityPositions(entityPositions);

  this->gameMap.renderEntityAtPos(entity);
}

void GameManager::updateEntityPosition(Entity *entity) {}

void GameManager::mainLoop() { this->gameMap.printMap(); }
