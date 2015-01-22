#pragma once
#include "board.hpp"

struct Random_AI {
    State piece;

    Pos get_move(Board const& board);

    void note_new_game();
    void note_victory(Board const& board);
    void note_defeat(Board const& board);
    void note_draw(Board const& board);
};
