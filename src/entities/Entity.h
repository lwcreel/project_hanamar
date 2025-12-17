#ifndef ENTITY_H
#define ENTITY_H

#include "../utils/Position.h"
#include <string>

class Entity {
private:
protected:
  int id;

public:
  Position position;
  int health = 10;
  std::string entityIcon;

  Entity();
  Entity(int x, int y, int id);
  Entity(int x, int y, int id, std::string entityIcon);

  Position getPosition();
  void setPosition(Position position);
  int getX();
  int getY();
  int getId();
  int getHealth();
  void setX(int x);
  void setY(int y);
  void setHealth(int health);

  void MoveLeft();
  void MoveRight();
  void MoveUp();
  void MoveDown();
};

#endif
