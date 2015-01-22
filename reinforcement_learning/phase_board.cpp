#include "phase_board.hpp"

namespace {
unsigned get_pos(Board const& board) {
    unsigned result = 0;
    for (int i = 0; i < 3; ++i)
        for (int j = 0; j < 3; ++j)
            result = 3*result + (unsigned)board.data[i][j];
    return result;
}
}

float PhaseBoard::get_value(Board const& board) const {
    return data[get_pos(board)];
}

void PhaseBoard::set_value(Board const& board, float value) {
    data[get_pos(board)] = value;
}
