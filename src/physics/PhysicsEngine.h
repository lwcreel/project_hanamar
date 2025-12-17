#ifndef PHYSICS_ENGINE_H
#define PHYSICS_ENGINE_H

#include "../entities/Entity.h"
#include "../utils/Position.h"
#include <string>
#include <unordered_map>

class PhysicsEngine {

  Position upperBound;
  Position lowerBound;

  // the positions of the entities with the entityID as a key
  std::unordered_map<int, Position> entityPositions;

public:
  PhysicsEngine();
  PhysicsEngine(Position upperBound, Position lowerBound,
                std::unordered_map<int, Position> entities);
  bool isOutOfBounds(Position position);
  Position entityCollision(Entity *entity1, Entity *entity2);
  std::unordered_map<int, Position> getEntityPositions();
  void setEntityPositions(std::unordered_map<int, Position> newEntityPositions);
};

#endif
