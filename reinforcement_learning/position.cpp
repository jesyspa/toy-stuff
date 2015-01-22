#include "position.hpp"
#include <iostream>

std::ostream& operator<<(std::ostream& os, Pos pos) {
    return os << pos.i << ' ' << pos.j;
}
