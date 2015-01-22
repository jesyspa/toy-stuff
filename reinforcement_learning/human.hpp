#pragma once
#include "board.hpp"
#include "position.hpp"

struct Human {
    static const State piece = State::O;
    Pos get_move(Board const& board);
    void note_victory(Board const& board);
    void note_defeat(Board const& board);
    void note_draw(Board const& board);
};
