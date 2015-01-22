#include "board.hpp"
#include <cassert>
#include <iostream>

bool Board::is_empty(Pos pos) const {
    auto oob = [](int i) { return i < 0 || i >= 3; };
    return !oob(pos.i) && !oob(pos.j) && board[pos.i][pos.j] == State::None;
}
void Board::play(Pos pos, State new_state) {
    assert(is_empty(pos));
    board[pos.i][pos.j] = new_state;
}

State Board::check_winner() const { return State::None; }

bool Board::is_full() const {
    for (int i = 0; i < 3; ++i)
        for (int j = 0; j < 3; ++j)
            if (board[i][j] == State::None)
                return false;
    return true;
}

void Board::print() const {
    const char symbol[3] = {' ', 'X', 'O'};
    std::cout << "   +---+\n";
    for (int i = 2; i >= 0; --i) {
        std::cout << " " << i << " |";
        for (int j = 0; j < 3; ++j) {
            std::cout << symbol[(int)board[j][i]];
        }
        std::cout << "|\n";
    }
    std::cout << "   +---+\n";
    std::cout << "    ";
    for (int i = 0; i < 3; ++i)
        std::cout << i;
    std::cout << '\n';
}
