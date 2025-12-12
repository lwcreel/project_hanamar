#include "Entity.h"

Entity::Entity() {
  this->x = 0;
  this->y = 0;
};

Entity::Entity(int x, int y) {
  this->x = x;
  this->y = y;
  this->entityIcon = "!";
}

Entity::Entity(int x, int y, std::string entityIcon) {

  this->x = x;
  this->y = y;
  this->entityIcon = entityIcon;
}

int Entity::getX() { return this->x; }

int Entity::getY() { return this->y; }

int Entity::getHealth() { return this->health; }

void Entity::setX(int x) { this->x = x; }

void Entity::setY(int y) { this->y = y; }

void Entity::setHealth(int health) { this->health = health; }

void Entity::MoveLeft() { this->x--; }

void Entity::MoveRight() { this->x++; }

void Entity::MoveUp() { this->y++; }

void Entity::MoveDown() { this->y--; }
