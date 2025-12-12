#ifndef GAME_MAP_H
#define GAME_MAP_H

#include <string>
#include <vector>

class GameMap {

  int map_boundary_x_;
  int map_boundary_y_;
  std::string map_horizontal_border_;
  std::string map_row_no_player_;
  std::vector<std::vector<char>> map_;

  bool OutOfBounds(int x_pos, int y_pos);

public:
  GameMap(int map_size_x, int map_size_y);
  void PrintMap();
  void PrintUserPosition(int user_position_x, int user_position_y);
  void PrintMobPosition(int mob_position_x, int mob_position_y,
                        std::string mob_icon);
};

#endif
