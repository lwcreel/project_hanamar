#include "entities/mobs/Mob.h"
#include "entities/player/Player.h"
#include "management/GameManager.h"
#include <iostream>
#include <unistd.h>
#include <unordered_map>

int main() {
  Player *player = new Player();
  Mob *mob = new Mob(7, 12, 1, "X");

  std::unordered_map<int, Entity *> temp = {{0, player}, {1, mob}};

  GameManager *manager = new GameManager(20, 20, temp);

  char command;

  std::cout << "Welcome to Project Hanamar!" << std::endl;

  while (true) {

    manager->mainLoop();
    std::cout << "Use WASD + Enter to Move and CTRL + C to Exit" << std::endl;
    std::cin >> command;

    switch (command) {
    case 'w':
      player->Entity::MoveUp();
      break;
    case 'a':
      player->Entity::MoveLeft();
      break;
    case 's':
      player->Entity::MoveDown();
      break;
    case 'd':
      player->Entity::MoveRight();
      break;
    }
    system("clear");
  }

  return 0;
}
