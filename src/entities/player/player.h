#ifndef PLAYER_H
#define PLAYER_H

#include "../entity.h"

// TODO: Move to a parent Entity class

class Player : public Entity {
public:
    Player()
        : Entity()
    {
        Entity::pos_x_ = 0;
        Entity::pos_y_ = 0;
        Entity::entity_icon_ = "@";
    };

    Player(int pos_x, int pos_y, std::string icon)
        : Entity(pos_x, pos_y, icon) { };
};

#endif
