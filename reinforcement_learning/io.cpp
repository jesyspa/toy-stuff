#include "io.hpp"
#include <cassert>

namespace {
std::ostream* active_out = &std::cout;
}

bool output_enabled() {
    return bool(active_out);
}

std::ostream& out() {
    assert(active_out != nullptr && "cannot print without a stream");
    return *active_out;
}

void set_output_stream(std::ostream& os) {
    active_out = &os;
}

void disable_output() {
    active_out = nullptr;
}
