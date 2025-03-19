#ifndef ENTITY_H
#define ENTITY_H

#include <string>

class Entity {

public:
    int pos_x_;
    int pos_y_;
    int health_ = 10;
    std::string entity_icon_;

    Entity();
    Entity(int pos_x, int pos_y);
    Entity(int pos_x, int pos_y, std::string entity_icon);

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
