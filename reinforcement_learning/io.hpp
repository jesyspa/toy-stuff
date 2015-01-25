#pragma once
#include <iostream>

bool output_enabled();
std::ostream& out();
void set_output_stream(std::ostream& os);
void disable_output();

template<typename... Args>
void print(Args&&... args) {
    if (!output_enabled())
        return;
    auto& os = out();
    int dummy[] = {(static_cast<void>(os << args), 0)...};
    os << '\n';
    (void)dummy;
}
