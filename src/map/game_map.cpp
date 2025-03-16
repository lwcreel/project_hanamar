#include "game_map.h"
#include <iostream>

bool GameMap::OutOfBounds(int x_pos, int y_pos)
{
    return (x_pos >= map_boundary_x_ - 1 || x_pos < 0 || y_pos >= map_boundary_y_ || y_pos < 0);
}

GameMap::GameMap(int map_size_x, int map_size_y)
{
    this->map_boundary_x_ = map_size_x;
    this->map_boundary_y_ = map_size_y;

    for (int curr_position_x = 0; curr_position_x < this->map_boundary_x_; curr_position_x++) {
        this->map_horizontal_border_.append("-");
    }
    this->map_horizontal_border_.append("-");

    this->map_row_no_player_.append("|");
    for (int curr_x = 1; curr_x < this->map_boundary_x_; curr_x++) {
        this->map_row_no_player_.append(" ");
    }
    this->map_row_no_player_.append("|");
}

void GameMap::PrintMap()
{
    std::cout << map_horizontal_border_ << std::endl;

    for (int curr_y = 0; curr_y < map_boundary_y_; curr_y++) {
        std::cout << map_row_no_player_ << std::endl;
    }

    std::cout << map_horizontal_border_ << std::endl;
    return;
}

void GameMap::PrintUserPosition(int user_position_x, int user_position_y)
{
    if (OutOfBounds(user_position_x, user_position_y)) {
        std::cout << "ERROR: Out of Bounds!" << std::endl;
        return;
    }

    std::string curr_user_row;
    curr_user_row.append("|");

    for (int curr_x = 0; curr_x < map_boundary_x_ - 1; curr_x++) {
        if (curr_x != user_position_x)
            curr_user_row.append(" ");
        else
            curr_user_row.append("@");
    }

    curr_user_row.append("|");

    std::cout << map_horizontal_border_ << std::endl;

    for (int curr_y = 0; curr_y < map_boundary_y_; curr_y++) {
        if (user_position_y != curr_y) {
            std::cout << map_row_no_player_ << std::endl;
            continue;
        }
        std::cout << curr_user_row << std::endl;
    }

    std::cout << map_horizontal_border_ << std::endl;
    return;
}

void GameMap::PrintMobPosition(int mob_position_x, int mob_position_y, std::string mob_icon)
{

    if (OutOfBounds(mob_position_x, mob_position_y)) {
        std::cout << "ERROR: Out of Bounds!" << std::endl;
        return;
    }

    std::string curr_user_row;
    curr_user_row.append("|");

    for (int curr_x = 0; curr_x < map_boundary_x_ - 1; curr_x++) {
        if (curr_x != mob_position_x)
            curr_user_row.append(" ");
        else
            curr_user_row.append(mob_icon);
    }

    curr_user_row.append("|");

    std::cout << map_horizontal_border_ << std::endl;

    for (int curr_y = 0; curr_y < map_boundary_y_; curr_y++) {
        if (mob_position_y != curr_y) {
            std::cout << map_row_no_player_ << std::endl;
            continue;
        }
        std::cout << curr_user_row << std::endl;
    }

    std::cout << map_horizontal_border_ << std::endl;
    return;
}
