#include "phase_board.hpp"
#include "dihedral_4.hpp"
#include <algorithm>

namespace {
unsigned get_pos(Board const& board) {
    unsigned result = 0;
    for (int i = 0; i < 3; ++i)
        for (int j = 0; j < 3; ++j)
            result = 3*result + (unsigned)board.data[i][j];
    return result;
}

unsigned get_minimal_pos(Board const& board) {
    unsigned result = PhaseBoard::board_size;
    for (auto e : Dihedral4::elements)
        result = std::min(get_pos(e * board), result);
    return result;
}
}

PhaseBoard::PhaseBoard() {
    std::fill(std::begin(data), std::end(data), 0.5);
}


float const& PhaseBoard::operator[](Board const& board) const {
    return data[get_minimal_pos(board)];
}

float& PhaseBoard::operator[](Board const& board) {
    return data[get_minimal_pos(board)];
}

