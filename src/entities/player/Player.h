#ifndef PLAYER_H
#define PLAYER_H

#include "../Entity.h"

// TODO: Move to a parent Entity class

class Player : public Entity {

public:
  Player() : Entity() {
    Entity::position = {0, 0};
    Entity::id = 0;
    Entity::entityIcon = "@";
  };

  Player(int x, int y, int id, std::string icon) : Entity(x, y, id, icon) {};
};

#endif
