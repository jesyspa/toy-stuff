#include <iostream>
#include <functional>
#include <vector>

struct State;

typedef std::function<void(State&)> Instruction;

struct State {
    State(size_t data_size, std::vector<Instruction> insts) : pc(), sp(), instructions(insts), data(data_size) {}

    void jump_absolute(int n) {
        pc = n;
    }
    void jump_relative(int n) {
        pc += n;
    }
    void branch_absolute(int n) {
        data[sp+1] = pc;
        pc = n;
    }
    void return_from_branch() {
        pc = data[sp+1];
    }
    void push_stack(int n) {
        data[sp+n] = n;
        sp += n;
    }
    void pop_stack() {
        sp -= data[sp];
    }
    int& operator[](int n) {
        return data[sp + n];
    }

    void run() {
        while (0 <= pc && pc < static_cast<int>(instructions.size())) {
            instructions[pc](*this);
            pc++;
        }
    }

  private:
    int pc;
    int sp;
    std::vector<Instruction> instructions;
    std::vector<int> data;
};

Instruction jump_absolute(int n) {
    return [n](State& s) { s.jump_absolute(n); };
}

Instruction jump_relative(int n) {
    return [n](State& s) { s.jump_relative(n); };
}

Instruction branch_absolute(int n) {
    return [n](State& s) { s.branch_absolute(n); };
}

Instruction return_from_branch() {
    return [](State& s) { s.return_from_branch(); };
}

Instruction push_stack(int n) {
    return [n](State& s) { s.push_stack(n); };
}

Instruction pop_stack() {
    return [](State& s) { s.pop_stack(); };
}

Instruction set(int n, int x) {
    return [=](State& s) { s[n] = x; };
}

Instruction add(int a, int b, int target) {
    return [=](State& s) { s[target] = s[a] + s[b]; };
}

Instruction mul(int a, int b, int target) {
    return [=](State& s) { s[target] = s[a] * s[b]; };
}

Instruction output(int a) {
    return [=](State& s) { std::cout << "s[" << a << "] = " << s[a] << std::endl; };
}

Instruction jump_if_nonzero(int n, int distance) {
    return [=](State& s) { if (s[n]) s.jump_relative(distance); };
}

int main() {
    std::vector<Instruction> insts {
        // beginning of program
        set(3, 5), // set parameter to 5
        push_stack(1), // move to stack frame
        branch_absolute(5), // call factorial
        pop_stack(), // return to this stack frame
        output(0), // output the result of our computation
        jump_absolute(-2), // this forces a halt
        jump_if_nonzero(2, 2), // beginning of factorial; if n == 1
        set(-1, 1), // set return value to 1
        return_from_branch(), // return from factorial call
        set(3, -1), // store -1 so that we can operate on it
        add(2, 3, 6), // add s[2] and s[3] and put it in s[6] (to use as a parameter)
        push_stack(4), // prepare next factorial call
        branch_absolute(5), // call factorial
        pop_stack(), // return to this call
        mul(2, 3, -1), // multiply n and the result of the call and put it in the return value
        return_from_branch(), // return from factorial call
    };
    State state(1000, insts);
    state.run();
}
