use IO;
use Math;
use LinkedLists;
use List;

inline proc ndigits(x): int {
    return floor(log10(x) + 1): int;
}

inline proc split(x, n): (int, int) {
    const div = 10**n;
    return (x/div, x%div);
}

proc splitMany(x, n)  {
    var partitions: list(int);
    const div: int = 10**n;
    var rem = x;
    while rem > 0 {
        partitions.pushBack(rem % div);
        rem /= div;
    }
    return partitions;
}

iter readInput() {
    var st, en: int;
    while (readf("%i-%i", st, en)) {
        yield (st, en);
        if stdin.readByte() == 10 then break; // newline
    }
}

proc main() {
    var allInvalid = 0;
    var allInvalid2 = 0;
    for (st, en) in readInput() {
        for i in st..en {
            const n = ndigits(i);
            if n % 2 == 0 {
                const halves = split(i, n/2);
                if halves[0] == halves[1] {
                    allInvalid += i;
                }
            }

            for d in 1..(n/2) {
                var partitions = splitMany(i, d);
                const first = partitions.first;
                var all = true;
                var total = 0;
                for p in partitions {
                    total += ndigits(p);
                    all &= (p == first);
                }
                if partitions.size >= 2 && total == n && all {
                    allInvalid2 += i;
                    break;
                }
            }
        }
    }
    writeln("All invalid IDs #1: ", allInvalid);
    writeln("All invalid IDs #2: ", allInvalid2);
}

