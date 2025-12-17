#include "Entity.h"

Entity::Entity() {
  this->position = {0, 0};
  this->id = -1;
};

Entity::Entity(int x, int y, int id) {

  this->position = {x, y};
  this->id = id;
  this->entityIcon = "!";
}

Entity::Entity(int x, int y, int id, std::string entityIcon) {

  this->position = {x, y};
  this->id = id;
  this->entityIcon = entityIcon;
}

int Entity::getHealth() { return this->health; }

Position Entity::getPosition() { return this->position; }

void Entity::setPosition(Position position) { this->position = position; }

int Entity::getId() { return this->id; }

void Entity::setHealth(int health) { this->health = health; }

void Entity::MoveLeft() { this->position.x--; }

void Entity::MoveRight() { this->position.x++; }

void Entity::MoveUp() { this->position.y++; }

void Entity::MoveDown() { this->position.y--; }
