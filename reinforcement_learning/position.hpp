#pragma once
#include <iosfwd>

struct Pos {
    int i, j;
};

std::ostream& operator<<(std::ostream& os, Pos pos);
