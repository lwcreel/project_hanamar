#include "entities/player/player.h"
#include "map/game_map.h"
#include <iostream>
#include <unistd.h>

int main()
{
    GameMap* map = new GameMap(20, 20);
    Player* player = new Player();
    char command;

    std::cout << "Welcome to Project Hanamar!" << std::endl;

    while (true) {
        map->PrintUserPosition(player->get_pos_x_(), player->get_pos_y_());

        std::cout << "Use WASD + Enter to Move and CTRL + C to Exit" << std::endl;
        std::cin >> command;

        switch (command) {
        case 'w':
            player->MoveDown();
            break;
        case 'a':
            player->MoveLeft();
            break;
        case 's':
            player->MoveUp();
            break;
        case 'd':
            player->MoveRight();
            break;
        }
        system("clear");
    }

    return 0;
}
