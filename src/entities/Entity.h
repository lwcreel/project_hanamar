#ifndef ENTITY_H
#define ENTITY_H

#include <string>

class Entity {
  int id;

public:
  int x;
  int y;

  int health = 10;
  std::string entityIcon;

  Entity();
  Entity(int x, int y);
  Entity(int x, int y, std::string entityIcon);

  int getX();
  int getY();
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
