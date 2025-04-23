#include <cassert>
#include <cfloat>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <unordered_map>
#include <vector>

using namespace std;
using search_cache = unordered_map<int, unordered_map<int, long>>;

struct Point {
    long int x, y;
};

struct Machine {
    Point button_a, button_b, prize;
};

void print_machine(const Machine &m) {
    printf("Button A: (X+%ld, Y+%ld)\n", m.button_a.x, m.button_a.y);
    printf("Button B: (X+%ld, Y+%ld)\n", m.button_b.x, m.button_b.y);
    printf("Prize: (%ld, %ld)\n", m.prize.x, m.prize.y);
}

vector<Machine> read_input() {
    vector<Machine> machines;
    while (!feof(stdin)) {
        Machine m;
        int ret = 0;
        ret += scanf("Button A: X+%ld, Y+%ld\n", &m.button_a.x, &m.button_a.y);
        ret += scanf("Button B: X+%ld, Y+%ld\n", &m.button_b.x, &m.button_b.y);
        ret += scanf("Prize: X=%ld, Y=%ld\n", &m.prize.x, &m.prize.y);
        assert(ret == 6);
        machines.push_back(m);
    }
    return machines;
}

inline bool operator==(const Point& p1, const Point& p2) {
    return p1.x == p2.x && p1.y == p2.y;
}

inline Point operator+(const Point& p1, const Point& p2) {
    return { .x = p1.x + p2.x, .y = p1.y + p2.y };
}

inline bool almost_equal(double x, double y) {
    return abs(x - y) < 0.001;
}

inline bool exists(const search_cache& cache, int a, int b) {
    if (cache.find(a) == cend(cache)) return false;
    const auto& cache_a = cache.at(a);
    if (cache_a.find(b) == cend(cache_a)) return false;
    return true;
}

long search_recursive(Machine& machine, int press_a, int press_b, Point claw, search_cache& cache) {
    if (exists(cache, press_a, press_b))
        return cache[press_a][press_b];

    if (machine.prize == claw)
        return press_a * 3 + press_b;

    long cost = INT64_MAX;
    if (press_a < 100)
        cost = min(search_recursive(machine, press_a + 1, press_b, claw + machine.button_a, cache), cost);
    if (press_b < 100)
        cost = min(search_recursive(machine, press_a, press_b + 1, claw + machine.button_b, cache), cost);

    cache[press_a][press_b] = cost;
    return cost;
}

void inverse_mat2(const double mat[4], double inv[4]) {
    const double a = mat[0], b = mat[1], c = mat[2], d = mat[3];
    const double det = (a * d) - (b * c);
    inv[0] = +d / det; inv[1] = -b / det;
    inv[2] = -c / det; inv[3] = +a / det;
}

void mult_mat2_vec2(const double a[4], const double b[2], double x[2]) {
    x[0] = a[0] * b[0] + a[1] * b[1];
    x[1] = a[2] * b[0] + a[3] * b[1];
}

//ax=b
void solve_linear_system(const double a[4], const double b[2], double x[2]) {
    double inva[] = {a[0], a[1], a[2], a[3]};
    inverse_mat2(a, inva);
    mult_mat2_vec2(inva, b, x);
}

long search_linear_system(Machine& m) {
    const double a[] = {
        double(m.button_a.x), double(m.button_b.x),
        double(m.button_a.y), double(m.button_b.y)
    };
    const double b[] = { double(m.prize.x), double(m.prize.y) };
    double x[2] = { 0 };

    solve_linear_system(a, b, x);

    const long press_a = round(x[0]), press_b = round(x[1]);
    if (!almost_equal(press_a, x[0]) || !almost_equal(press_b, x[1]))
        return INT64_MAX;

    return press_a * 3 + press_b;
}

int main() {
    vector<Machine> machines = read_input();

    long total_cost_1 = 0;
    for (auto& machine: machines) {
        search_cache cache;
        const long cost = search_recursive(machine, 0, 0, { .x=0, .y=0 }, cache);
        if (cost < INT64_MAX) total_cost_1 += cost;
    }
    printf("cost = %ld\n\n", total_cost_1);

    long total_cost_2 = 0;
    for (auto& machine: machines) {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        const long cost = search_linear_system(machine);
        if (cost < INT64_MAX) total_cost_2 += cost;
    }
    printf("cost = %ld\n\n", total_cost_2);

    return 0;
}
