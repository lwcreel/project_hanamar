#include "GameManager.h"
#include <unordered_map>

GameManager::GameManager(int mapWidth, int mapHeight,
                         std::unordered_map<int, Entity *> entities)
    : gameMap(GameMap(mapWidth, mapHeight)), physicsEngine() {

  this->entities = entities;
}

void GameManager::spawnEntity(Entity *entity) {

  // Update PhysicsEngine
  std::unordered_map<int, Position> entityPositions =
      this->physicsEngine.getEntityPositions();
  entityPositions[entity->getId()] = {entity->getPosition().x,
                                      entity->getPosition().y};
  this->physicsEngine.setEntityPositions(entityPositions);

  this->gameMap.renderEntityAtPos(entity);
}

void GameManager::updateEntityPosition(Entity *entity) {

  std::unordered_map<int, Position> positions =
      this->physicsEngine.getEntityPositions();

  Position oldPosition = positions.at(entity->getId());

  if (this->physicsEngine.isOutOfBounds(entity->getPosition())) {
    entity->setPosition(oldPosition);
    return;
  }

  this->gameMap.renderAtPos(oldPosition, " ");

  positions.at(entity->getId()) = {entity->getPosition().x,
                                   entity->getPosition().y};

  this->physicsEngine.setEntityPositions(positions);
  this->gameMap.renderEntityAtPos(entity);
}

void GameManager::updateEntities() {
  for (const auto &[id, entity] : this->entities) {
    if (this->physicsEngine.isOutOfBounds(entity->position)) {
      return;
    }
    updateEntityPosition(entity);
  }
}

void GameManager::startMainLoop() {}

void GameManager::mainLoop() {

  outOfBoundsCheck();

  updateEntities();

  this->gameMap.printMap();
}

void GameManager::outOfBoundsCheck() {}
