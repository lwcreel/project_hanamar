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

bool PhysicsEngine::isOutOfBounds(int x, int y) {
  return !(x < this->upperBound.x || x >= lowerBound.x ||
           y < this->upperBound.y || y >= lowerBound.y);
}

std::unordered_map<int, Position> PhysicsEngine::getEntityPositions() {
  return this->entityPositions;
}

void PhysicsEngine::setEntityPositions(std::unordered_map<int, Position>) {
  return;
}
