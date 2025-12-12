#ifndef PHYSICS_ENGINE_H
#define PHYSICS_ENGINE_H

#include <string>
#include <unordered_map>
#include <utility>

class PhysicsEngine {

  std::unordered_map<int, std::pair<int, int>> entityPositions;
  std::pair<int, int> boundsX;
  std::pair<int, int> boundsY;

public:
  PhysicsEngine();
};

#endif
