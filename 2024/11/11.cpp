#include <cmath>
#include <cstdio>
#include <list>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

using namespace std;

list<unsigned long> read_input() {
    list<unsigned long> stones;
    int num;
    while (scanf("%d", &num) == 1) stones.push_back(num);
    return stones;
}

pair<unsigned long, unsigned long> split_number(unsigned long num,
                                                int pivot_digit) {
    const unsigned long halve_point = pow(10, pivot_digit);
    const unsigned long second = num % halve_point;
    const unsigned long first = num / halve_point;
    return make_pair(first, second);
}

void blink(list<unsigned long>& stones) {
    for (auto it = begin(stones); it != end(stones); ++it) {
        auto& num = *it;
        if (num == 0) {
            num = 1;
        } else {
            const int ndigits = int(log10(num)) + 1;
            if (ndigits % 2 == 0) {
                const auto& [first, second] = split_number(num, ndigits / 2);
                stones.insert(it, first);
                *it = second;
            } else {
                num *= 2024;
            }
        }
    }
}

unordered_map<unsigned long, unordered_map<int, unsigned long>> lazy_blink_cache;
unsigned long lazy_blink_stone_count(unsigned long num, int remain_blink) {
    if (remain_blink > 0) {
        const auto& cache_num = lazy_blink_cache[num];
        if (cache_num.find(remain_blink) != cache_num.end()) {
            return lazy_blink_cache[num][remain_blink];
        }

        if (num == 0) {
            const unsigned long count =
                lazy_blink_stone_count(1, remain_blink - 1);
            lazy_blink_cache[num][remain_blink] = count;
            return count;
        } else {
            const int ndigits = int(log10(num)) + 1;
            if (ndigits % 2 == 0) {
                const auto& [first, second] = split_number(num, ndigits / 2);
                const unsigned long count =
                    lazy_blink_stone_count(first, remain_blink - 1) +
                    lazy_blink_stone_count(second, remain_blink - 1);
                lazy_blink_cache[num][remain_blink] = count;
                return count;
            } else {
                const unsigned long count = lazy_blink_stone_count(num * 2024, remain_blink - 1);
                lazy_blink_cache[num][remain_blink] = count;
                return count;
            }
        }
    }

    return 1;
}

void print_stones(list<unsigned long>& stones) {
    for (auto n : stones) {
        printf("%lu ", n);
    }
    printf("\n");
}

int main() {
    list<unsigned long> stones = read_input();
    print_stones(stones);

    {
        list<unsigned long> blinked_stones(stones);
        const int nblinks = 25;
        for (int i = 0; i < nblinks; ++i) {
            blink(blinked_stones);
        }
        printf("\n%d blinks total stones: %lu\n", nblinks, blinked_stones.size());
    }

    {
        const int nblinks = 75;
        unsigned long sum = 0;
        for (auto stone : stones) {
            sum += lazy_blink_stone_count(stone, nblinks);
        }
        printf("\n%d blinks total stones: %lu\n", nblinks, sum);
    }

    return 0;
}
