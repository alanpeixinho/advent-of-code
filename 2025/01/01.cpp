#include <cstdio>
#include <cmath>
#include <cassert>

using namespace std;

struct Move {
    char dir;
    short steps;
};

Move read_input() {
    char dir;
    short steps;
    assert(scanf("%c%hd\n", &dir, &steps) == 2);
    return {
        .dir = dir,
        .steps = steps
    };
}

inline short modulo(short x, short mod) {
    return ((x % mod) + mod) % mod;
}

int main() {
    short cur_pos = 50;
    int occurrences1 = 0, occurrences2 = 0;
    while (!feof(stdin)) {
        const Move m = read_input();
        if (m.steps == 0) continue;

       const short dir = m.dir == 'L' ? -m.steps : +m.steps;
       const short next_pos = modulo(cur_pos + dir, 100);

       if (next_pos == 0)
           occurrences1++;

       occurrences2 += abs(dir / 100);
       if (cur_pos != 0) {
           if (dir < 0 && (next_pos > cur_pos || next_pos == 0))
               occurrences2++;
           if (dir > 0 && next_pos < cur_pos)
               occurrences2++;
       }

        cur_pos = next_pos;
    }
    printf("Password 1: %d\n", occurrences1);
    printf("Password 2: %d\n", occurrences2);
    return 0;
}
