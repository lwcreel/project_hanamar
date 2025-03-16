#ifndef GAME_MAP_H
#define GAME_MAP_H

#include <string>

class GameMap {

    int map_boundary_x_;
    int map_boundary_y_;
    std::string map_horizontal_border_;
    std::string map_row_no_player_;

    bool OutOfBounds(int x_pos, int y_pos);

public:
    GameMap(int map_size_x, int map_size_y);
    void PrintMap();
    void PrintUserPosition(int user_position_x, int user_position_y);
};

#endif
