#include <cstddef>
#include <cstdio>
#include <cstring>
#include <vector>

using namespace std;

inline int ctoi(char c) {
    return c - '0';
}

inline void trim_newline(char* s) {
    s[strcspn(s, "\n")] = '\0';
}

int max_voltage(const char* bank) {
    int start = 0, end = 0;
    for (int i = 0; bank[i + 1]; ++i) {
        if (bank[i] > bank[start])
            start = i;
    }
    end = start + 1;
    for (int i = start + 1; bank[i]; ++i) {
        if (bank[i] > bank[end]) {
            end = i;
        }
    }
    return ctoi(bank[start]) * 10 + ctoi(bank[end]);
}

long max_voltage(const char* bank, int n) {
    vector<int> index(n, 0);

    for (int b = 0; b < n; ++b) {
        int cur_index = b >= 1 ? index[b-1] + 1 : 0;
        for (int i = cur_index; bank[i + (n-b) - 1]; ++i) {
            if (bank[i] > bank[cur_index])
                cur_index = i;
            index[b] = cur_index;
        }
    }

    long total = 0;
    for (int idx : index) {
        total = 10 * total + ctoi(bank[idx]);
    }
    return total;
}

int main() {

    size_t size;
    char *bank = new char[1024];

    int total = 0;
    long total_12 = 0;

    while (!feof(stdin)) {
        if (getline(&bank, &size, stdin) == -1)
            break;
        trim_newline(bank);
        total += max_voltage(bank);
        total_12 += max_voltage(bank, 12);
    }

    printf("Total output voltage for 2 batteries: %d\n", total);
    printf("Total output voltage for 12 batteries: %ld\n", total_12);

    delete[] bank;

    return 0;
}
