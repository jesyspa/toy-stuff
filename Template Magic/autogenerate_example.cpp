#include "autogenerate.hpp"
#include <iostream>
#include <string>

struct Example {
    int x, y;
    std::string z;
};

template <>
struct enable_operator_synthesis<Example> : std::true_type {
    using members = sequence<SYNTH_MEMBER_ACCESSOR(Example::x), SYNTH_MEMBER_ACCESSOR(Example::y),
                             SYNTH_MEMBER_ACCESSOR(Example::z)>;
};

int main() {
    Example a{1, 2, "Hello"};
    Example b{1, 3, "World"};
    std::cout << "False: " << (a == b) << ", " << (a > b) << '\n';
    std::cout << "True: " << (a == a) << ", " << (a < b) << '\n';
}
