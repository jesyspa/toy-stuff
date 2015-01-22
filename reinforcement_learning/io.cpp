#include "io.hpp"
#include <iostream>
#include <stdexcept>

namespace {
std::ostream* active_out = &std::cout;
}

std::ostream& out() {
    return *active_out;
}

void set_out(std::ostream& os) {
    active_out = &os;
}
