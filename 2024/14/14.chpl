use IO;
use IO.FormattedIO;
use LinkedLists;

const WIDTH: int = 101, HEIGHT: int = 103;

proc modulo(x: int, y: int) {
    const m = x % y;
    return if m < 0 then y + m else m;
}

proc quadrant(pos: (int, int)): int {
    const h_middle = HEIGHT / 2, w_middle = WIDTH / 2;

    if pos[0] == w_middle || pos[1] == h_middle then return -1;

    const qx = if pos[0] < w_middle then 0 else 1;
    const qy = if pos[1] < h_middle then 0 else 1;

    return qx * 2 + qy;
}

proc read_input(): [] ((int, int), (int, int)) {
    var list: LinkedList(((int, int), (int, int)));
    var vx, vy, px, py: int;
    while (readf("p=%i,%i v=%i,%i\n", px, py, vx, vy)) {
        list.push_back(((px, py), (vx, vy)));
    }
    return list.these();
}

proc printRoom(robots) {
    var field: [0..#HEIGHT, 0..#WIDTH] int = 0;
    for robot in robots {
        field[robot[0][1], robot[0][0]] += 1;
    }
    for (i, j) in {0..#HEIGHT, 0..#WIDTH} {
        write(if field[i, j] > 0 then 'X' else ' ');
        if (j == WIDTH - 1) then writeln();
    }
}

proc main() {
    var robots = read_input();

    for i in 1..100 {
        for (pos, vel) in robots {
            pos = (modulo(pos[0] + vel[0], WIDTH), modulo(pos[1] + vel[1], HEIGHT));
        }
    }

    var quadrants: [-1..3] int;
    for robot in robots {
        quadrants[quadrant(robot[0])] += 1;
    }

    writeln('quadrants: ', quadrants);
    writeln('safety factor: ', * reduce quadrants[0..]);
    writeln('number of robots: ', + reduce quadrants);

    /*
     * This part was tricky, had to look on community board
     * the hint was too look for anomalies (many robots grouped in a quadrant)
     * and visualize check for the trees pattern
     */
    for i in 1..1000000 {
        for (pos, vel) in robots {
            pos = (modulo(pos[0] + vel[0], WIDTH), modulo(pos[1] + vel[1], HEIGHT));
        }
        var quadrants: [-1..3] int;
        for(pos, _) in robots {
            quadrants[quadrant(pos)] += 1;
        }
        if (max reduce for q in quadrants[0..] do q) > 250 {
            writef("==========%i==========\n\n", i);
            printRoom(robots);
        }
    }
}
