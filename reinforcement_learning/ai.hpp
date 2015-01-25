#pragma once
#include "board.hpp"
#include "position.hpp"
#include "phase_board.hpp"
#include <boost/optional.hpp>
#include <utility>
#include <vector>
#include <random>

struct AI {
    static constexpr State piece = State::X;
    static constexpr double default_alpha = 0.5;
    static constexpr double default_alpha_step = 0.999999;
    static constexpr double default_base_explore_chance = 0.6;
    static constexpr double default_explore_step = 0.5;

    double alpha = default_alpha;
    double alpha_step = default_alpha_step;
    double base_explore_chance = default_base_explore_chance;
    double explore_step = default_explore_step;

    AI();
    Pos get_move(Board const& board);
    void note_new_game();
    void note_victory(Board const& board);
    void note_defeat(Board const& board);
    void note_draw(Board const& board);

    void print_info() const;

private:
    double explore_chance = base_explore_chance;

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
