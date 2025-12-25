#include <array>
#include <cassert>
#include <cstdio>
#include <vector>

using namespace std;

constexpr int NUM_SHAPES = 6;
constexpr int DIM = 3;

using Gift = array<array<char, DIM>, DIM>;

Gift readGift() {
    int idx = 0;
    if (scanf("%d:\n", &idx) < 1) return {};
    Gift gift;
    for (int i = 0; i < DIM; ++i) {
        for (int j = 0; j < DIM; ++j) {
            gift[i][j] = getc(stdin);
        }
        assert(getc(stdin) == '\n');
    }
    assert(getc(stdin) == '\n');
    return gift;
}

pair<pair<int, int>, array<int, NUM_SHAPES>> readRegion() {
    int width, height;
    if (scanf("%dx%d: ", &width, &height) < 2) return {};
    array<int, NUM_SHAPES> quantity;
    for (int i = 0; i < NUM_SHAPES; ++i) {
        if (scanf("%d", &quantity[i]) < 1) return {};
    }

    return {
        {width, height},
        quantity
    };
}

int area(const Gift& gift) {
    int a = 0;
    for (int i = 0; i < DIM; ++i)
        for (int j = 0; j < DIM; ++j)
            a += gift[i][j] == '#';
    return a;
}

int main() {
    vector<Gift> gifts;
    for (int i = 0; i < NUM_SHAPES; ++i) {
        gifts.push_back(readGift());
    }

    int total = 0;
    int impossible = 0;

    while (true) {
        const auto& [region_dim, quantity] = readRegion();
        const auto& [width, height] = region_dim;
        if (width == 0 && height == 0)
            break;

        const int region_area = width * height;
        int gift_area = 0;
        for (int i = 0; i < NUM_SHAPES; ++i) {
            gift_area += area(gifts[i]) * quantity[i];
        }

        if (gift_area > region_area)
            impossible++;
        total++;
    }
    printf("# possible: %d/%d\n", total - impossible, total);
}
