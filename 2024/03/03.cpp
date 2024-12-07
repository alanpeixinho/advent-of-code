#include <cassert>
#include <cctype>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>

using  namespace std;

bool match_pattern(const char* pattern, const char* memory) {
    size_t j = 0;
    for(size_t i = 0; pattern[i]; ++i) {
        if (pattern[i] == '$' && isdigit(memory[j]))
            while (isdigit(memory[j])) ++j;
        else if (pattern[i] == memory[j]) ++j;
        else return false;
    }
    return true;
}

int64_t mul_parse(const char *memory) {
    int x1 = 0, x2 = 0;
    sscanf(memory, "mul(%d,%d)", &x1, &x2);
    return x1 * x2;
}

int slurp(FILE* f, char* memory, size_t &size) {
    fread((void*)memory, sizeof(char), size, f);
    size = ftell(f);
    return 0;
}

int main() {
    const char* mul_pattern = "mul($,$)";
    const char* do_pattern = "do()";
    const char* dont_pattern = "don't()";

    size_t memsize = 4096;
    char *memory = new char[memsize];

    bool enabled = true;

    assert(slurp(stdin, memory, memsize) == 0);

    int64_t total = 0;
    for (size_t i = 0; i < memsize; ++i) {
        if (match_pattern(do_pattern, &memory[i])) {
            enabled = true;
        } else if (match_pattern(dont_pattern, &memory[i])) {
            enabled = false;
        } else if (enabled && match_pattern(mul_pattern, &memory[i])) {
            total += mul_parse(&memory[i]);
        }
    }

    delete [] memory;

    printf("total = %ld\n", total);
    return 0;
}
