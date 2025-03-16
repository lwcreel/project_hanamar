#include "mob.h"

Mob::Mob()
{
    this->pos_x_ = 0;
    this->pos_y_ = 0;
};

Mob::Mob(int pos_x, int pos_y)
{
    this->pos_x_ = pos_x;
    this->pos_y_ = pos_y;
    this->mob_icon_ = "!";
}

Mob::Mob(int pos_x, int pos_y, std::string mob_icon)
{

    this->pos_x_ = pos_x;
    this->pos_y_ = pos_y;
    this->mob_icon_ = mob_icon;
}

int Mob::get_pos_x_()
{
    return this->pos_x_;
}

int Mob::get_pos_y_()
{
    return this->pos_y_;
}

int Mob::get_health_()
{
    return this->health_;
}

void Mob::set_pos_x(int pos_x)
{
    this->pos_x_ = pos_x;
}

void Mob::set_pos_y(int pos_y)
{
    this->pos_y_ = pos_y;
}

void Mob::set_health_(int health)
{
    this->health_ = health;
}

void Mob::MoveLeft()
{
    this->pos_x_--;
}

void Mob::MoveRight()
{
    this->pos_x_++;
}

void Mob::MoveUp()
{

    this->pos_y_++;
}

void Mob::MoveDown()
{

    this->pos_y_--;
}
