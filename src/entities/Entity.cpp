#include "Entity.h"

Entity::Entity() {
  this->x = 0;
  this->y = 0;
  this->id = -1;
};

Entity::Entity(int x, int y, int id) {
  this->x = x;
  this->y = y;
  this->id = id;
  this->entityIcon = "!";
}

Entity::Entity(int x, int y, int id, std::string entityIcon) {

  this->x = x;
  this->y = y;
  this->id = id;
  this->entityIcon = entityIcon;
}

int Entity::getX() { return this->x; }

int Entity::getY() { return this->y; }

int Entity::getHealth() { return this->health; }

void Entity::setX(int x) { this->x = x; }

void Entity::setY(int y) { this->y = y; }

int Entity::getId() { return this->id; }

void Entity::setHealth(int health) { this->health = health; }

void Entity::MoveLeft() { this->x--; }

void Entity::MoveRight() { this->x++; }

void Entity::MoveUp() { this->y++; }

void Entity::MoveDown() { this->y--; }
