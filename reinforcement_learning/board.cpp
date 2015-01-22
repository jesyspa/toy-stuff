#include "board.hpp"
#include "io.hpp"
#include <cassert>
#include <iostream>

bool Board::is_empty(Pos pos) const {
    auto oob = [](int i) { return i < 0 || i >= 3; };
    return !oob(pos.i) && !oob(pos.j) && data[pos.i][pos.j] == State::None;
}
void Board::play(Pos pos, State new_state) {
    assert(is_empty(pos));
    data[pos.i][pos.j] = new_state;
}

State Board::check_winner() const { return State::None; }

bool Board::is_full() const {
    for (int i = 0; i < 3; ++i)
        for (int j = 0; j < 3; ++j)
            if (data[i][j] == State::None)
                return false;
    return true;
}

void Board::print() const {
    auto& os = out();
    const char symbol[3] = {' ', 'X', 'O'};
    os << "   +---+\n";
    for (int i = 2; i >= 0; --i) {
        os << " " << i << " |";
        for (int j = 0; j < 3; ++j) {
            os << symbol[(int)data[j][i]];
        }
        os << "|\n";
    }
    os << "   +---+\n";
    os << "    ";
    for (int i = 0; i < 3; ++i)
        os << i;
    os << '\n';
}
