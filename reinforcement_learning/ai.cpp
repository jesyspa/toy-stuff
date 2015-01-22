#include "ai.hpp"
#include <random>
#include <iostream>

std::mt19937 gen(std::random_device{}());

Pos AI::get_move(Board const& board) {
    std::uniform_int_distribution<> dist(0, 2);
    Pos move{-1, -1};
    while (!board.is_empty(move)) {
        move.i = dist(gen);
        move.j = dist(gen);
    }
    std::cout << "[AI] I shall play... " << move << "\n";
    return move;
}

void AI::note_victory(Board const&) {}
void AI::note_defeat(Board const&) {}
void AI::note_draw(Board const&) {}
