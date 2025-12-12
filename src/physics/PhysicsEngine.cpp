#include "PhysicsEngine.h"

PhysicsEngine::PhysicsEngine() {

  this->entityPositions = {{0, {0, 0}}, {1, {5, 5}}};
  this->boundsX = {0, 20};
  this->boundsY = {0, 20};
}
