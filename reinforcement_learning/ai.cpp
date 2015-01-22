#include "ai.hpp"
#include "io.hpp"
#include <random>
#include <iostream>

AI::AI() : gen(std::random_device{}()) {}

Pos AI::get_move(Board const& board) {
    std::uniform_int_distribution<> dist(0, 2);
    Pos move{-1, -1};
    while (!board.is_empty(move)) {
        move.i = dist(gen);
        move.j = dist(gen);
    }
    out() << "[Reinforcement AI] I shall play... " << move << "\n";
    return move;
}

void AI::note_new_game() {
    states.clear();
}

void AI::note_victory(Board const& board) {
    phase_data.set_value(board, 1);
}

void AI::note_defeat(Board const& board) {
    phase_data.set_value(board, -2);
}

void AI::note_draw(Board const& board) {
    phase_data.set_value(board, -1);
}
