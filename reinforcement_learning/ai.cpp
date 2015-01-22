#include "ai.hpp"
#include "io.hpp"
#include <random>
#include <iostream>

AI::AI() : gen(std::random_device{}()) {}

Pos AI::get_move(Board const& board) {
    std::uniform_real_distribution<> rdist(0, 1);
    Pos move;
    if (rdist(gen) < 0.2)
        move = play_exploratory(board);
    else
        move = play_best(board);
    states.push_back(board);
    Board local = board;
    local.play(move, piece);
    states.push_back(local);
    if (!silent) {
        out() << "[Reinforcement AI] I shall play... " << move << "\n";
        out() << "[Reinforcement AI] This move has value " << phase_data[local] << ".\n";
    }
    return move;
}

Pos AI::play_exploratory(Board const& board) {
    if (!silent)
        out() << "[Reinforcement AI] Let me experiment...\n";
    std::uniform_int_distribution<> board_dist(0, 2);
    Pos move{-1, -1};
    while (!board.is_empty(move)) {
        move.i = board_dist(gen);
        move.j = board_dist(gen);
    }
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
            auto value = phase_data[local];
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

void AI::update_intermediate(Board board) {
    for (auto state : states) {
        phase_data[state] = (phase_data[state] + phase_data[board]) / 2;
        board = state;
    }
}

void AI::note_new_game() {
    games += 1;
    states.clear();
}

void AI::note_victory(Board const& board) {
    wins += 1;
    phase_data[board] = 1;
    update_intermediate(board);
}

void AI::note_defeat(Board const& board) {
    losses += 1;
    phase_data[board] = -2;
    update_intermediate(board);
}

void AI::note_draw(Board const& board) {
    phase_data[board] = -1;
    update_intermediate(board);
}

void AI::print_info() const {
    int total = 0;
    for (unsigned i = 0; i < phase_data.board_size; ++i)
        if (phase_data.data[i] != 0)
            total += 1;
    out() << "[Reinforcement AI] I have info about " << total << " positions.\n";
    out() << "[Reinforcement AI] I have played " << games << " games, of which I won " << wins << " and lost " << losses << ".\n";
}

