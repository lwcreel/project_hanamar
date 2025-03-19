#ifndef MOB_H
#define MOB_H

#include "../entity.h"

class Mob : public Entity {

public:
    Mob(int pos_x, int pos_y, std::string icon)
        : Entity(pos_x, pos_y, icon) { };
};

#endif
