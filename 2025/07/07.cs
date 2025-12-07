IEnumerable<(int, int)> Iter2D(int rows, int cols) {
    for (int i = 0; i < rows; ++i) {
        for (int j = 0; j < cols; ++j) {
            yield return (i, j);
        }
    }
}

IEnumerable<(int, int)> Domain(char [,] matrix) {
    return Iter2D(matrix.GetLength(0), matrix.GetLength(1));
}

char[,] ReadInput() {
    List<string> lines = new List<string>();
    while (true) {
        string? line = Console.ReadLine();
        if (line == null || line.Trim() == string.Empty)
            break;
        lines.Add(line);
    }
    char [,] diagram = new char[lines.Count, lines[0].Length];
    foreach (var (i, j) in Domain(diagram)) {
        diagram[i, j] = lines[i][j];
    }
    return diagram;
}

List<(int, int)> FindSplitters(char[,] diagram) {
    List<(int, int)> l = new List<(int, int)>();
    foreach (var (i, j) in Domain(diagram)) {
        if (diagram[i, j] == '^') {
            l.Add((i, j));
        }
    }
    return l;
}

int CountActivatedSplitters(char[,] diagram) {
    List<(int, int)> splitters = FindSplitters(diagram);
    int activatedSplitters = 0;
    foreach (var (cur_i, cur_j) in splitters) {
        int lastRow = 0;
        int lastCol = 0;
        foreach (var (i, j) in splitters) {
            if (i >= cur_i)
                continue;
            if (j < cur_j - 1 || j > cur_j + 1)
                continue;
            if (i > lastRow) {
                lastRow = i;
                lastCol = j;
            }
        }
        if (lastCol != cur_j) {
            activatedSplitters++;
        }
    }
    return activatedSplitters;
}

(int, int) FindStart(char[,] diagram) {
    foreach (var (i, j) in Domain(diagram)) {
        if (diagram[i, j] == 'S')
            return (i, j);
    }
    return (-1, -1);
}

long CountAllPossibilities(char[,] diagram) {
    var (row, col) = FindStart(diagram);
    return Dfs(row, col, diagram, new Dictionary<(int, int), long>());
}

long Dfs(int row, int column, char[,] diagram, Dictionary<(int, int), long> cache) {

    if (cache.ContainsKey((row, column)))   return cache[(row, column)];

    if (column < 0 || column >= diagram.GetLength(1))   return 0;

    long n = 0;
    if (row >= diagram.GetLength(0) - 1) {
        n = 1;
    } else if (diagram[row + 1, column] != '^') {
        n = Dfs(row + 1, column, diagram, cache);
    } else {
        n += Dfs(row + 1, column - 1, diagram, cache);
        n += Dfs(row + 1, column + 1, diagram, cache);
    }

    cache[(row, column)] = n;
    return n;
}

char[,] diagram = ReadInput();
Console.WriteLine("# of activated splitters: {0}", CountActivatedSplitters(diagram));

Console.WriteLine("# of quantum possibilities: {0}", CountAllPossibilities(diagram));
