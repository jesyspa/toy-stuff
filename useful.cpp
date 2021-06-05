#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <utility>

using i64 = long long int;

template<typename T>
T read() { T t; std::cin >> t; return t; }
#define READ(T, x) T const x = read<T>()

template<typename T>
struct ShowVecImpl {
  ShowVecImpl(std::vector<T> const& v) : v(&v) {}
  std::vector<T> const* v;
};

template<typename T>
ShowVecImpl<T> show(std::vector<T> const& v) { return ShowVecImpl<T>(v); }

template<typename T>
std::ostream& operator<<(std::ostream& os, ShowVecImpl<T> pvi) {
  auto const& v = *pvi.v;
  os << '{';
  if (!v.empty()) {
    os << v[0];
    for (size_t i = 1; i < v.size(); ++i) {
      os << ", " << v[i];
    }
  }
  os << '}';
  return os;
}

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
  std::ios_base::sync_with_stdio(false);
  std::cin.tie(0);
  std::cout.tie(0);
  READ(int, T);
  for (int i = 0; i < T; ++i) {
    go();
  }
}
