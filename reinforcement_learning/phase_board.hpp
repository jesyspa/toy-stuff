#pragma once
#include "board.hpp"

struct PhaseBoard {
    static const unsigned board_size = 19683; // 3^9
    float data[board_size] = {};

    float get_value(Board const& board) const;
    void set_value(Board const& board, float value);
};
