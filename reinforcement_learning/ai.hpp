#pragma once
#include "board.hpp"
#include "position.hpp"
#include "phase_board.hpp"
#include <boost/optional.hpp>
#include <utility>
#include <vector>
#include <random>

struct AI {
    static const State piece = State::X;

    AI();
    Pos get_move(Board const& board);
    void note_new_game();
    void note_victory(Board const& board);
    void note_defeat(Board const& board);
    void note_draw(Board const& board);

    void print_info() const;

private:
    double base_explore_chance = 0.8;
    double explore_chance = 0.8;
    double explore_step = 0.5;
    double alpha = 0.5;
    double alpha_step = 0.999999;

    int games = 0;
    int wins = 0;
    int losses = 0;
    PhaseBoard phase_data;
    boost::optional<std::pair<Board, Board>> last_move;
    std::mt19937 gen;

    Pos choose_move(Board const& board);

    Pos play_exploratory(Board const& board);
    Pos play_best(Board const& board);

    void update_last_move(Board const& board);
};
