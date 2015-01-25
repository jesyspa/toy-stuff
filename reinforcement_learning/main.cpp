#include "board.hpp"
#include "ai.hpp"
#include "random_ai.hpp"
#include "human.hpp"
#include "io.hpp"
#include <iostream>
#include <sstream>
#include <string>
#include <limits>

template<typename Player>
bool get_move(Player& player, Board& board) {
    auto move = player.get_move(board);
    board.play(move, player.piece);

    if (board.check_winner() != State::None) {
        player.note_victory(board);
        return true;
    }

    return false;
}

template<typename PlayerX, typename PlayerO>
void play_round(PlayerX& player_x, PlayerO& player_o) {
    Board board;
    player_x.note_new_game();
    player_o.note_new_game();
    while (std::cin) {
        auto ai_won = get_move(player_x, board);
        if (ai_won) {
            player_o.note_defeat(board);
            return;
        }

        if (board.is_full()) {
            player_x.note_draw(board);
            player_o.note_draw(board);
            return;
        }

        auto human_won = get_move(player_o, board);
        if (human_won) {
            player_x.note_defeat(board);
            return;
        }

        if (board.is_full()) {
            player_x.note_draw(board);
            player_o.note_draw(board);
            return;
        }
    }
}

template<typename Player>
void play_against_random(Player& player, State random_piece, int count) {
    Random_AI rand_ai{random_piece};
    auto& old_out = out();
    disable_output();
    for (int i = 0; i < count; ++i) {
        play_round(player, rand_ai);
    }
    set_output_stream(old_out);
}

template<typename T>
T get_optional_input(std::string const& prompt, T def) {
    std::cout << prompt << " [" << def << "]: ";
    T val;
    std::string str;
    std::getline(std::cin, str);
    std::istringstream ss(str);
    if (ss >> val)
        return val;
    return def;
}

int main(int argc, char**) try {
    AI ai;
    int training_count = 1'000'000;
    if (argc > 1) {
        training_count = get_optional_input("Training sessions", training_count);
        ai.alpha = get_optional_input("alpha", AI::default_alpha);
        ai.alpha_step = get_optional_input("alpha step", AI::default_alpha_step);
        ai.base_explore_chance = get_optional_input("base explore chance", AI::default_base_explore_chance);
        ai.explore_step = get_optional_input("explore step", AI::default_explore_step);
    }

    std::cout << "Training...\n";
    play_against_random(ai, State::O, training_count);

    Human human;
    while (std::cin) {
        ai.print_info();
        play_round(ai, human);
    }
    std::cout << "Bye!\n";
}
catch (std::exception& e) {
    std::cerr << "Error: " << e.what() << "\n";
}
