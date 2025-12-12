#ifndef MOB_H
#define MOB_H

#include "../Entity.h"

class Mob : public Entity {

public:
  Mob(int x, int y, std::string icon) : Entity(x, y, icon) {};
};

#endif
