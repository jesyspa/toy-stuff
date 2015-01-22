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

State Board::check_winner() const { 
    for (int i = 0; i < 3; ++i) {
        if (data[i][0] != State::None && data[i][0] == data[i][1] && data[i][1] == data[i][2])
            return data[i][0];
        if (data[0][i] != State::None && data[0][i] == data[1][i] && data[1][i] == data[2][i])
            return data[0][i];
    }
    if (data[0][0] != State::None && data[0][0] == data[1][1] && data[1][1] == data[2][2])
        return data[0][0];
    if (data[2][0] != State::None && data[2][0] == data[1][1] && data[1][1] == data[0][2])
        return data[2][0];
    return State::None;
}

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
