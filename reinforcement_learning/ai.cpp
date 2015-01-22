#include "ai.hpp"
#include "io.hpp"
#include <random>
#include <iostream>

AI::AI() : gen(std::random_device{}()) {}

Pos AI::get_move(Board const& board) {
    std::uniform_real_distribution<> rdist(0, 1);
    if (rdist(gen) < 0.2)
        return play_exploratory(board);
    else
        return play_best(board);
}

Pos AI::play_exploratory(Board const& board) {
    std::uniform_int_distribution<> board_dist(0, 2);
    Pos move{-1, -1};
    while (!board.is_empty(move)) {
        move.i = board_dist(gen);
        move.j = board_dist(gen);
    }
    out() << "[Reinforcement AI] I shall play... " << move << "\n";
    return move;
}

Pos AI::play_best(Board const& board) {
    Pos best;
    float best_value = -100;
    for (int i = 0; i < 3; ++i) {
        for (int j = 0; j < 3; ++j) {
            if (!board.is_empty({i, j}))
                continue;
            Board local = board;
            local.play({i, j}, piece);
            auto value = phase_data.get_value(local);
            if (value > best_value) {
                best_value = value;
                best = Pos{i, j};
            }
        }
    }
    if (best_value == -100)
        return play_exploratory(board);
    return best;
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
