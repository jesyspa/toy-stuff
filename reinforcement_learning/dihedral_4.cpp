#include "dihedral_4.hpp"
#include <iostream>

namespace {
    std::array<Dihedral4, 8> dihedrals() {
        std::array<Dihedral4, 8> elems;
        for (int i = 0; i < 8; ++i) {
            elems[i].flip = i % 2;
            elems[i].rotate = i / 2;
        }
        return elems;
    }
}

std::array<Dihedral4, 8> Dihedral4::elements = dihedrals();

Dihedral4 operator*(Dihedral4 lhs, Dihedral4 rhs) {
    auto lr = lhs.rotate;
    auto rr = lhs.flip ? -rhs.rotate : rhs.rotate;
    return Dihedral4{lhs.flip != rhs.flip, (4 + lr + rr) % 4};
}

std::ostream& operator<<(std::ostream& os, Dihedral4 e) {
    os << e.rotate;
    if (e.flip)
        os << 's';
    return os;
}
