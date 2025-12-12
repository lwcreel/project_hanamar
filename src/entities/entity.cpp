#include "entity.h"

Entity::Entity() {
  this->pos_x_ = 0;
  this->pos_y_ = 0;
};

Entity::Entity(int pos_x, int pos_y) {
  this->pos_x_ = pos_x;
  this->pos_y_ = pos_y;
  this->entity_icon_ = "!";
}

Entity::Entity(int pos_x, int pos_y, std::string entity_icon) {

  this->pos_x_ = pos_x;
  this->pos_y_ = pos_y;
  this->entity_icon_ = entity_icon;
}

int Entity::get_pos_x_() { return this->pos_x_; }

int Entity::get_pos_y_() { return this->pos_y_; }

int Entity::get_health_() { return this->health_; }

void Entity::set_pos_x(int pos_x) { this->pos_x_ = pos_x; }

void Entity::set_pos_y(int pos_y) { this->pos_y_ = pos_y; }

void Entity::set_health_(int health) { this->health_ = health; }

void Entity::MoveLeft() { this->pos_x_--; }

void Entity::MoveRight() { this->pos_x_++; }

void Entity::MoveUp() { this->pos_y_++; }

void Entity::MoveDown() { this->pos_y_--; }
