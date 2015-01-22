#pragma once
#include "board.hpp"

struct PhaseBoard {
    static const unsigned board_size = 19683; // 3^9
    float data[board_size] = {};

    float const& operator[](Board const& board) const;
    float& operator[](Board const& board);
};
