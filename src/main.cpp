#include "entities/mobs/Mob.h"
#include "entities/player/Player.h"
#include "management/GameManager.h"
#include <iostream>
#include <unistd.h>

int main() {
  Player *player = new Player();
  Mob *mob = new Mob(7, 12, 1, "X");
  GameManager *manager = new GameManager(20, 20);

  char command;

  std::cout << "Welcome to Project Hanamar!" << std::endl;

  while (true) {

    manager->spawnEntity(player);
    manager->spawnEntity(mob);
    manager->mainLoop();
    std::cout << "Use WASD + Enter to Move and CTRL + C to Exit" << std::endl;
    std::cin >> command;

    switch (command) {
    case 'w':
      player->Entity::MoveDown();
      break;
    case 'a':
      player->Entity::MoveLeft();
      break;
    case 's':
      player->Entity::MoveUp();
      break;
    case 'd':
      player->Entity::MoveRight();
      break;
    }
    system("clear");
  }

  return 0;
}
