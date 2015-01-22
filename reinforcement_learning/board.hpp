#pragma once
#include "position.hpp"

enum class State {
    None, X, O
};

struct Board {
    State board[3][3] = {};

    bool is_empty(Pos pos) const;
    void play(Pos pos, State new_state);

    State check_winner() const;
    bool is_full() const;
    void print() const;
};
