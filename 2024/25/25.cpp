#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <utility>
#include <vector>

constexpr int NPINS = 5;
constexpr int HEIGHT = 7;

enum SchematicType {
    LOCK,
    KEY
};

struct Schematic {
    SchematicType type;
    int8_t sizes[NPINS];
};

using namespace std;

pair<vector<Schematic>, vector<Schematic>> read_schematics_input() {

    size_t linesize = 128;
    char* line = new char[linesize];

    vector<Schematic> locks, keys;
    while (true) {
        assert(getline(&line, &linesize, stdin));

        if (feof(stdin)) break;

        Schematic schematic;
        schematic.type = line[0] == '.' ? SchematicType::KEY : SchematicType::LOCK;
        const int init = line[0] == '.' ? HEIGHT - 2 : 0;
        for (int j = 0; j < NPINS; ++j)
            schematic.sizes[j] = init;

        for (int i = 1; i < HEIGHT; ++i) {
            assert(getline(&line, &linesize, stdin));
            for (int j = 0; j < NPINS; ++j) {
                if (schematic.type == SchematicType::KEY) {
                    if (line[j] == '.') schematic.sizes[j]--;
                } else {
                    if (line[j] == '#') schematic.sizes[j]++;
                }
            }
        }
        getchar(); //read new line

        if (schematic.type == SchematicType::LOCK)
            locks.push_back(schematic);
        else
            keys.push_back(schematic);
    }

    delete[] line;
    return make_pair(locks, keys);
}

void print_schematic(const Schematic& schematic) {
    printf("%s -> ", schematic.type == SchematicType::LOCK ? "Lock" : "Key");
    for (int j = 0; j < NPINS; ++j) {
        printf("%d ", schematic.sizes[j]);
    }
    printf("\n");
}

bool lock_key_fit(const Schematic& lock, const Schematic& key) {
    assert(lock.type == SchematicType::LOCK && key.type == SchematicType::KEY);
    for (int j = 0; j < NPINS; ++j) {
        if ((lock.sizes[j] + key.sizes[j]) > 5)
            return false;
    }
    return true;
}

int main() {
    const auto&[locks, keys] = read_schematics_input();
    int count = 0;
    for (const auto& key : keys) {
        for (const auto& lock : locks) {
            if (lock_key_fit(lock, key)) count++;
        }
    }
    printf("# of valid combinations: %d\n", count);
}
