#include "board.hpp"
#include "ai.hpp"
#include "random_ai.hpp"
#include "human.hpp"
#include "io.hpp"
#include <iostream>
#include <sstream>

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

int main() try {
    AI ai;

    std::cout << "First playing a few thousand games against a random AI...\n";
    play_against_random(ai, State::O, 1'000'000);

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
