#include "board.hpp"
#include "ai.hpp"
#include "human.hpp"
#include <iostream>

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

void play_round(AI& ai, Human& human) {
    Board board;
    while (std::cin) {
        auto ai_won = get_move(ai, board);
        if (ai_won) {
            human.note_defeat(board);
            return;
        }

        if (board.is_full()) {
            ai.note_draw(board);
            human.note_draw(board);
            return;
        }

        auto human_won = get_move(human, board);
        if (human_won) {
            ai.note_defeat(board);
            return;
        }

        if (board.is_full()) {
            ai.note_draw(board);
            human.note_draw(board);
            return;
        }
    }
}

int main() try {
    AI ai;
    Human human;
    while (std::cin)
        play_round(ai, human);
    std::cout << "Bye!\n";
}
catch (std::exception& e) {
    std::cerr << "Error: " << e.what() << "\n";
}
