#include "ai.hpp"
#include "io.hpp"
#include <random>
#include <iostream>

AI::AI() : last_move{}, gen(std::random_device{}()) {}

Pos AI::get_move(Board const& board) {
    update_last_move(board);
    auto move = choose_move(board);

    explore_chance *= explore_step;

    Board local = board;
    local.play(move, piece);
    last_move = std::make_pair(local, board);

    print("[Reinforcement AI] I shall play... ", move);
    print("[Reinforcement AI] This move has value ", phase_data[local], '.');
    return move;
}

Pos AI::choose_move(Board const& board) {
    std::uniform_real_distribution<> rdist(0, 1);
    if (rdist(gen) < explore_chance)
        return play_exploratory(board);
    else
        return play_best(board);
}

Pos AI::play_exploratory(Board const& board) {
    print("[Reinforcement AI] Let me experiment...");
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

void AI::update_last_move(Board const& board) {
    if (!last_move)
        return;
    auto before = last_move->first;
    auto after = last_move->second;
    phase_data[after] += alpha * (phase_data[board] - phase_data[after]);
    phase_data[before] += alpha * (phase_data[board] - phase_data[before]);
    alpha *= alpha_step;
}

void AI::note_new_game() {
    games += 1;
    last_move = {};
    explore_chance = base_explore_chance;
}

void AI::note_victory(Board const& board) {
    wins += 1;
    phase_data[board] = 1;
    update_last_move(board);
}

void AI::note_defeat(Board const& board) {
    losses += 1;
    phase_data[board] = -1;
    update_last_move(board);
}

void AI::note_draw(Board const& board) {
    phase_data[board] = 0;
    update_last_move(board);
}

void AI::print_info() const {
    print("[Reinforcement AI] I have played ", games, " games, of which I won ", wins, " and lost ", losses, ".\n");
}

