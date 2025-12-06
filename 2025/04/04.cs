IEnumerable<(int, int)> Iter2D(int rows, int cols) {
    for (int i = 0; i < rows; ++i) {
        for (int j = 0; j < cols; ++j) {
            yield return (i,j);
        }
    }
}

IEnumerable<(int, int)> Domain(char [,] matrix) {
    return Iter2D(matrix.GetLength(0), matrix.GetLength(1));
}

char[,] ReadInput() {
    List<string> lines = new List<string>();;
    while (true) {
        string? line = Console.ReadLine();
        if (line == null || line.Trim() == string.Empty)
            break;
        lines.Add(line);
    }
    char[,] map = new char[lines.Count, lines[0].Length];
    foreach (var (i, j) in Domain(map))
        map[i, j] = lines[i][j];
    return map;
}

bool InBounds<T>(int i, int j, T[,] map) {
    return i >= 0 && j >= 0 && i < map.GetLength(0) && j < map.GetLength(1);
}

int RemoveAccessible(int [,] count, char [,] map) {
    int accessible = 0;
    foreach (var (i, j) in Domain(map)) {
        if (map[i,j] == '@' && count[i,j] <= 3) {
            accessible++;
            map[i, j] = 'X';
        }
    }
    return accessible;
}

int[,] NeighborCount(char [,] map) {

    int [,] count = new int[map.GetLength(0), map.GetLength(1)];

    (int, int)[] neighbors = {
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    };

    foreach(var (i, j) in Domain(map)) {
        if (map[i, j] == '@') {
            foreach (var (ni, nj) in neighbors) {
                if (InBounds(i+ni, j+nj, count))
                    count[i + ni, j+nj]++;
            }
        }
    }

    return count;
}

char[,] map = ReadInput();
int removed = RemoveAccessible(NeighborCount(map), map);
Console.WriteLine($"# of removed first run: {removed}");

int total_removed = removed;
while (true) {
    int cur_removed = RemoveAccessible(NeighborCount(map), map);
    if (cur_removed == 0)
        break;
    total_removed += cur_removed;
}
Console.WriteLine($"# of removed total: {total_removed}");

