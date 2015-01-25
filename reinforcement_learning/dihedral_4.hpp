#pragma once
#include <array>
#include <iosfwd>

struct Dihedral4 {
    bool flip;
    int rotate;

    static std::array<Dihedral4, 8> elements;
};

Dihedral4 operator*(Dihedral4 lhs, Dihedral4 rhs);
std::ostream& operator<<(std::ostream& os, Dihedral4 e);
