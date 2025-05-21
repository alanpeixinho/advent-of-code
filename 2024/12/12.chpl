use IO;
use LinkedLists;

// union find
record DisjointSet {
    type dtype;
    var set: domain(dtype);
    var parent: [set] dtype;
}

proc ref DisjointSet.find(x: dtype): dtype throws {
    if !set.contains(x) { throw new Error("socorro");  }
    if parent[x] != x then parent[x] = find(parent[x]);
    return parent[x];
}

proc ref DisjointSet.join(x: dtype, y: dtype) {
    const parentX = find(x);
    const parentY = find(y);
    if parentY != parentX then parent[parentY] = parentX;
}

proc ref DisjointSet.insert(x: dtype) {
    set += x;
    parent[x] = x;
}

proc readInput(): [] uint(8) {
    var list: LinkedList(string);
    for line in stdin.lines() {
        list.push_back(line.strip());
    }
    const (height, width) = (list.size, list.first().size);
    var data: [0..#height, 0..#width] uint(8);
    for (i, line) in zip(0..#height, list) {
        data[i, ..] = line.bytes();
    }
    return data;
}

proc connectedComponents(const ref data: [?D] uint(8)): [D] int(32)
    where D.rank == 2 {
    var labels: [D] int(32) = 0;
    var nlabels: int(32) = 1;

    var unionFind: DisjointSet(int(32));

    //first pass
    for (i, j) in D {
        const neighLabels = for (ni, nj) in { (i-1, j), (i, j-1) } do
            if D.contains(ni, nj) && data[ni, nj] == data[i, j] then labels[ni, nj];
        if neighLabels.isEmpty() {
            labels[i, j] = nlabels;
            unionFind.insert(nlabels);
            nlabels += 1;
        } else {
            labels[i, j] = min reduce neighLabels;
            for l in neighLabels do unionFind.join(labels[i, j], l);
        }
    }

    //second pass
    for (i, j) in D {
        if labels[i, j] > 0 then labels[i, j] = unionFind.find(labels[i, j]);
    }

    return labels;
}

proc perimeter(const ref data: [?D] int(32)): [] int(32)
    where D.rank == 2 {
    var pixels: domain(int(32));
    var perimeter: [pixels] int(32);

    for (i, j) in D {
        var sum: int(32) = 0;
        for (ni, nj) in { (i-1, j), (i+1, j), (i, j-1), (i, j+1) } {
            if !D.contains(ni, nj) || data[ni, nj] != data[i, j] then sum += 1;
        }
        pixels += data[i, j];
        perimeter[data[i, j]] += sum;
    }

    return perimeter;
}

proc area(const ref data: [] int(32)): [] int(32) {
    var pixels: domain(int(32));
    var area: [pixels] int(32);
    for v in data {
        pixels += v;
        area[v] += 1;
    }
    return area;
}

proc sides(const ref data: [?D] int(32)): [] int(32)
    where D.rank == 2 {
    var pixels: domain(int(32));
    var corners: [pixels] int(32); //number of corners is the same as number of sides

    const directions = [
        [ (0, 1), (1, 0), (1, 1) ],      //right-down
        [ (0, 1), (-1, 0), (-1, 1) ],     //right-up
        [ (0, -1), (1, 0), (1, -1) ],     //left-down
        [ (0, -1), (-1, 0), (-1, -1) ],     //left-up
    ];

    for (i, j) in D {
        var sum: int(32) = 0;
        for dir in directions {
            const check = for d in 0..2 do
                (!D.contains((i, j) + dir[d]) || data[(i, j) + dir[d]] != data[(i, j)]);
            if check[0] && check[1] { // convex corners
                sum += 1;
            }
            if !check[0] && !check[1] && check[2] { // concave corners
                sum += 1;
            }
        }
        pixels += data[i, j];
        corners[data[i, j]] += sum;
    }

    return corners;
}

proc main() {
    const garden = readInput();
    const labels = connectedComponents(garden);

    const gardenPerimeter = perimeter(labels);
    const gardenArea = area(labels);
    const gardenSides = sides(labels);
    assert(gardenArea.domain == gardenPerimeter.domain, "something is fishy");

    const total = + reduce for (a, p) in zip(gardenArea, gardenPerimeter) do a * p;
    writeln("Total price: ", total);

    const bulkTotal = + reduce for (a, s) in zip(gardenArea, gardenSides) do a * s;
    writeln("Bulk total price: ", bulkTotal);
}
