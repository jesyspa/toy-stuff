#pragma once
#include "position.hpp"
#include "dihedral_4.hpp"

enum class State {
    None, X, O
};

struct Board {
    State data[3][3] = {};

    bool is_empty(Pos pos) const;
    void play(Pos pos, State new_state);

    State check_winner() const;
    bool is_full() const;
    void print() const;
};

Board operator*(Dihedral4 e, Board board);
