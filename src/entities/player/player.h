#ifndef PLAYER_H
#define PLAYER_H

class Player {

    int pos_x_;
    int pos_y_;
    int health_ = 10;

public:
    Player();
    Player(int pos_x, int pos_y);

    int get_pos_x_();
    int get_pos_y_();
    int get_health_();
    void set_pos_x(int pos_x);
    void set_pos_y(int pos_y);
    void set_health_(int health);

    void MoveLeft();
    void MoveRight();
    void MoveUp();
    void MoveDown();
};

#endif
