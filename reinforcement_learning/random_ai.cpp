#include "random_ai.hpp"
#include "io.hpp"
#include <random>
#include <iostream>

namespace {
std::mt19937 gen(std::random_device{}());
}

Pos Random_AI::get_move(Board const& board) {
    std::uniform_int_distribution<> dist(0, 2);
    Pos move{-1, -1};
    while (!board.is_empty(move)) {
        move.i = dist(gen);
        move.j = dist(gen);
    }
    print("[Random AI] I shall play... ", move);
    return move;
}

void Random_AI::note_new_game() {}
void Random_AI::note_victory(Board const&) {}
void Random_AI::note_defeat(Board const&) {}
void Random_AI::note_draw(Board const&) {}
