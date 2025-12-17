#include "PhysicsEngine.h"

PhysicsEngine::PhysicsEngine() {

  this->entityPositions = {{0, {0, 0}}, {1, {5, 5}}};
  this->upperBound = {0, 20};
  this->lowerBound = {0, 20};
}

PhysicsEngine::PhysicsEngine(Position upperBound, Position lowerBound,
                             std::unordered_map<int, Position> entities) {

  this->upperBound = upperBound;
  this->lowerBound = lowerBound;
  this->entityPositions = entities;
}

bool PhysicsEngine::isOutOfBounds(Position position) {
  return !(position.x < this->upperBound.x || position.x >= lowerBound.x ||
           position.y < this->upperBound.y || position.y >= lowerBound.y);
}

std::unordered_map<int, Position> PhysicsEngine::getEntityPositions() {
  return this->entityPositions;
}

void PhysicsEngine::setEntityPositions(
    std::unordered_map<int, Position> newPositions) {
  this->entityPositions = newPositions;
}
