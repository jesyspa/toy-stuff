#pragma once
#include "board.hpp"
#include "position.hpp"

struct AI {
    static const State piece = State::X;
    Pos get_move(Board const& board);
    void note_victory(Board const& board);
    void note_defeat(Board const& board);
    void note_draw(Board const& board);
};
