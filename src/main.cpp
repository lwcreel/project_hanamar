#include "entities/mobs/Mob.h"
#include "entities/player/Player.h"
#include "map/GameMap.h"
#include <iostream>
#include <unistd.h>

int main() {

  GameMap map = GameMap(20, 20);
  Player player = Player(1, 1, "@");
  Mob mob = Mob(7, 12, "X");
  char command;

  std::cout << "Welcome to Project Hanamar!" << std::endl;

  while (true) {
    map.PrintUserPosition(player.get_pos_x_(), player.get_pos_y_());

    std::cout << "Use WASD + Enter to Move and CTRL + C to Exit" << std::endl;
    std::cin >> command;

    switch (command) {
    case 'w':
      player.Entity::MoveDown();
      break;
    case 'a':
      player.Entity::MoveLeft();
      break;
    case 's':
      player.Entity::MoveUp();
      break;
    case 'd':
      player.Entity::MoveRight();
      break;
    }
    system("clear");
  }

  return 0;
}
