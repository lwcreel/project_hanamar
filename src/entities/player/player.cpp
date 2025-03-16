#include "player.h"

Player::Player()
{
    this->pos_x_ = 0;
    this->pos_y_ = 0;
};

Player::Player(int pos_x, int pos_y)
{
    this->pos_x_ = pos_x;
    this->pos_y_ = pos_y;
}

int Player::get_pos_x_()
{
    return this->pos_x_;
}

int Player::get_pos_y_()
{
    return this->pos_y_;
}

int Player::get_health_()
{
    return this->health_;
}

void Player::set_pos_x(int pos_x)
{
    this->pos_x_ = pos_x;
}

void Player::set_pos_y(int pos_y)
{
    this->pos_y_ = pos_y;
}

void Player::set_health_(int health)
{
    this->health_ = health;
}

void Player::MoveLeft()
{
    this->pos_x_--;
}

void Player::MoveRight()
{
    this->pos_x_++;
}

void Player::MoveUp()
{

    this->pos_y_++;
}

void Player::MoveDown()
{

    this->pos_y_--;
}
