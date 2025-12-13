#ifndef MOB_H
#define MOB_H

#include "../Entity.h"

class Mob : public Entity {

public:
  Mob() : Entity() {};
  Mob(int x, int y, int id, std::string icon) : Entity(x, y, id, icon) {};
};

#endif
