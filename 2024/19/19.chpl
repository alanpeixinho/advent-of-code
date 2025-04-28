use IO;
use LinkedLists;

proc readInput() {
    var patterns: LinkedList(string);
    for p in stdin.readLine(stripNewline=true).split(", ") {
        patterns.push_back(p);
    }
    stdin.advanceThrough("\n");

    var towels: LinkedList(string);
    var t: string;
    while stdin.readLine(t, stripNewline=true) {
        towels.push_back(t);
    };

    const patArr: [0..#patterns.size] string = patterns;
    const towArr: [0..#towels.size] string = towels;

    return (patArr, towArr);
}

var cacheKeys: domain(string, parSafe=false);
var cache: [cacheKeys] int;
proc dfsRec(const ref towel: string, const ref patterns): int {
    if cacheKeys.contains(towel) then return cache[towel];
    if towel.isEmpty() then return 1;
    var total = 0;
    for p in patterns {
        if towel.startsWith(p) {
            const count = dfsRec(towel[p.size..], patterns);
            total += count;
        }
    }
    cacheKeys += towel;
    cache[towel] = total;
    return total;
}

proc main() {
    var (patterns, towels) = readInput();
    const count = (+ reduce for t in towels do if dfsRec(t, patterns) then 1 else 0);
    writeln("# of possible: ", count);
    const sum = (+ reduce for t in towels do dfsRec(t, patterns));
    writeln("# of combinations: ", sum);
}
