#include "human.hpp"
#include <iostream>
#include <stdexcept>

Pos Human::get_move(Board const& board) {
    std::cout << "The board is as follows:\n";
    board.print();
    std::cout << "Enter your move: ";
    int i, j;
    while (std::cin >> i >> j) {
        Pos move{i, j};
        if (board.is_empty(move))
            return move;
        std::cout << "Invalid move.\n";
    }
    throw std::runtime_error{"User failed to provide input."};
}

namespace {
void print_final_state(Board const& board) {
    std::cout << "The final board state is:\n\n";
    board.print();
    std::cout << "\n";
}
}

void Human::note_victory(Board const& board) {
    std::cout << "You have won!\n";
    print_final_state(board);
}

void Human::note_defeat(Board const& board) {
    std::cout << "You have been defeated!\n";
    print_final_state(board);
}

void Human::note_draw(Board const& board) {
    std::cout << "The game concluded without a winner.\n";
    print_final_state(board);
}
