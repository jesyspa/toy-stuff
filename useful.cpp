#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <utility>

using i64 = long long int;

template<typename T>
T read() { T t; std::cin >> t; return t; }
#define READ(T, x) const T x = read<T>()

struct Node {
  std::vector<size_t> edges;
};

struct Nodes {
  Nodes(size_t N) : nodes(N) {}

  void add(size_t u, size_t v) {
    nodes[u].edges.push_back(v);
    nodes[v].edges.push_back(u);
  }

  void solve(size_t start) {}

  std::vector<Node> nodes;
};

void go() {}

int main() {
  READ(int, T);
  for (int i = 0; i < T; ++i) {
    go();
  }
}
