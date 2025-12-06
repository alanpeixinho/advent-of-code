#include <iostream>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <vector>
#include <string>

using namespace std;

inline int ndigits(long x){
    return floor(log10(x) + 1);
}

inline pair<int, int> split(long x, int n) {
    const int div = pow(10, n);
    return { x/div, x%div };
}

vector<long> split_many(long x, int n)  {
    vector<long> partitions;
    const long div = pow(10, n);
    long rem = x;
    while (rem > 0) {
        partitions.push_back(rem % div);
        rem /= div;
    }
    return partitions;
}

vector<pair<long, long>> read_input() {
    vector<pair<long, long>> l;
    long st, en;
    string buffer;
    while(!feof(stdin)) {
        getline(cin, buffer, '-');
        st = stol(buffer);
        getline(cin, buffer, ',');
        en = stol(buffer);
        l.push_back({st, en});
    }
    return l;
}

int main() {
    long allInvalid = 0;
    long allInvalid2 = 0;
    pair<long, long> r;
    for (const auto& [st, en] : read_input()) {
        for(long i = st;  i <= en; ++i) {
            const int n = ndigits(i);
            if (n % 2 == 0) {
                const auto halves = split(i, n/2);
                if (halves.first == halves.second) {
                    allInvalid += i;
                }
            }

            for (int d = 1; d <=n/2; ++d) {
                const auto partitions = split_many(i, d);
                const auto first = partitions.front();
                bool all = true;
                int total = 0;
                for (const auto& p : partitions) {
                    all &= p == first;
                    total += ndigits(p);
                }
                if (partitions.size() >= 2 && total == n && all) {
                    allInvalid2 += i;
                    break;
                }
            }
        }
    }
    printf("All invalid IDs #1: %ld\n", allInvalid);
    printf("All invalid Ids #2: %ld\n", allInvalid2);
    return 0;
}

