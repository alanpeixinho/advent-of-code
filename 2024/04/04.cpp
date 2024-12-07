#include <cstddef>
#include <cstdio>
#include <cstring>

using namespace std;

const size_t MAX_TROWS = 256, MAX_TCOLS = 256;

void ini_table(char table[MAX_TROWS][MAX_TCOLS])
{
    for (int row = 0; row < MAX_TROWS; ++row) {
        memset(table[row], '\0', MAX_TCOLS);
    }
}

int read_table(char table[MAX_TROWS][MAX_TCOLS], FILE* f)
{
    int character = 0;
    size_t row = 0, col = 0;
    while (true) {
        character = fgetc(f);
        if (character == EOF)
            break;
        if (character == '\n') {
            ++row;
            col = 0;
        } else {
            table[row][col] = character;
            ++col;
        }
    }

    return 0;
}

int search_pattern(const char* pattern,
    int x, int y,
    const char table[MAX_TROWS][MAX_TCOLS],
    int dx, int dy)
{
    int found = 0;

    int curx = x, cury = y;
    int pi = 0;
    while (curx >= 0 && curx < MAX_TROWS && cury >= 0 && cury < MAX_TCOLS && pattern[pi] == table[curx][cury]) {
        curx += dx;
        cury += dy;
        if (pattern[++pi] == '\0') {
            ++found;
            break;
        }
    }

    return found;
}

int main()
{

    char table[MAX_TROWS][MAX_TCOLS];

    ini_table(table);
    read_table(table, stdin);

    // fake pattern search 01
    {
        const char* pattern = "XMAS";
        const int dirx[] = { -1, -1, -1, 0, 0, 0, 1, 1, 1 };
        const int diry[] = { -1, 0, 1, -1, 0, 1, -1, 0, 1 };
        int total_found = 0;
        for (int x = 0; x < MAX_TROWS; ++x) {
            for (int y = 0; y < MAX_TCOLS; ++y) {
                for (int di = 0; di < 9; ++di) {
                    const int dx = dirx[di];
                    const int dy = diry[di];
                    const int found = search_pattern(pattern, x, y, table,
                        dx, dy);
                    total_found += found;
                }
            }
        }

        printf("found: %d\n", total_found);
    }

    // true pattern search 02
    {

        const char* pattern = "MAS";
        const int poffset = strlen(pattern) - 1;

        const int dirx[] = { -1, -1, 1, 1 };
        const int diry[] = { -1, 1, -1, 1 };
        int total_found = 0;
        for (int x = 0; x < MAX_TROWS; ++x) {
            for (int y = 0; y < MAX_TCOLS; ++y) {
                for (int di = 0; di < 4; ++di) {
                    const int dx = dirx[di];
                    const int dy = diry[di];
                    const int found = search_pattern(pattern, x, y, table, dx, dy);
                    if (found) {
                        if (search_pattern(pattern, x + dx * poffset, y, table, -dx, dy) || search_pattern(pattern, x, y + dy * poffset, table, dx, -dy)) {
                            ++total_found;
                        }
                    }
                }
            }
        }

        total_found /= 2; // we counted all twice

        printf("true found: %d\n", total_found);
    }

    return 0;
}
