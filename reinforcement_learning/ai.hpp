#pragma once
#include "board.hpp"
#include "position.hpp"
#include "phase_board.hpp"
#include <vector>
#include <random>

struct AI {
    static const State piece = State::X;
    bool silent = false;

    AI();
    Pos get_move(Board const& board);
    void note_new_game();
    void note_victory(Board const& board);
    void note_defeat(Board const& board);
    void note_draw(Board const& board);

    void print_info() const;

private:
    int games = 0;
    int wins = 0;
    int losses = 0;
    PhaseBoard phase_data;
    std::vector<Board> states;
    std::mt19937 gen;

    Pos play_exploratory(Board const& board);
    Pos play_best(Board const& board);

    void update_intermediate(Board board);
};
