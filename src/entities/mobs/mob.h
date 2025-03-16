#ifndef MOB_H
#define MOB_H

#include <string>

// TODO: Move to a parent entity class

class Mob {

    int pos_x_;
    int pos_y_;
    int health_ = 10;
    std::string mob_icon_;

public:
    Mob();
    Mob(int pos_x, int pos_y);
    Mob(int pos_x, int pos_y, std::string mob_icon);

    int get_pos_x_();
    int get_pos_y_();
    int get_health_();
    void set_pos_x(int pos_x);
    void set_pos_y(int pos_y);
    void set_health_(int health);

    void MoveLeft();
    void MoveRight();
    void MoveUp();
    void MoveDown();
};

#endif
