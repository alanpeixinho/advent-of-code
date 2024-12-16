use std::{
    collections::HashSet,
    io::{stdin, Read},
    thread::{sleep, sleep_ms},
};

fn read_input() -> (Vec<Vec<char>>, Vec<char>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in stdin().lines() {
        let rows: Vec<char> = line.expect("error on read").trim().chars().collect();
        if rows.is_empty() {
            break;
        }
        map.push(rows);
    }

    let mut directions: String = String::new();
    stdin()
        .read_to_string(&mut directions)
        .expect("error on read directions");

    (map, directions.chars().filter(|c| *c != '\n').collect())
}

fn empty_space_ahead(map: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> Option<(i32, i32)> {
    let mut i = 1;

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    loop {
        let r = pos.0 + i * dir.0;
        let c = pos.1 + i * dir.1;

        if (r < 0 || r >= rows) || (c < 0 || c >= cols) {
            break;
        }

        let cur = map[r as usize][c as usize];

        if cur == '.' {
            return Some((r, c));
        } else if cur == '#' {
            break;
        }

        i += 1;
    }
    None
}

fn move_robot(map: &mut Vec<Vec<char>>, pos: &mut (i32, i32), dir: (i32, i32)) {
    if let Some(emptyspace) = empty_space_ahead(map, *pos, dir) {
        let newpos = (pos.0 + dir.0, pos.1 + dir.1);
        map[pos.0 as usize][pos.1 as usize] = '.';
        map[emptyspace.0 as usize][emptyspace.1 as usize] = 'O';
        *pos = newpos;
        map[pos.0 as usize][pos.1 as usize] = '@';
    }
}

fn is_inside(map: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    (pos.0 >= 0 && pos.0 < rows) && (pos.1 >= 0 && pos.1 < cols)
}

fn push_ahead(map: &mut Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    if is_horizontal_move(dir) {
        if let Some(empty_space) = empty_space_ahead(map, pos, dir) {
            push_ahead_horiz(map, pos, dir, empty_space);
            return true;
        }
    } else if is_vertical_move(dir) {
        let ok = check_ahead_vert(&map, pos, dir);
        //println!("{:?}", ok);
        if ok {
            let mut visited = HashSet::new();
            push_ahead_vert(map, pos, dir, &mut visited);
            return true;
        }
    }
    false
}

fn check_ahead_vert(map: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    match map[next_pos.0 as usize][next_pos.1 as usize] {
        '.' => true,
        ']' => {
            check_ahead_vert(map, (next_pos.0, pos.1), dir)
                && check_ahead_vert(map, (pos.0 + dir.0, pos.1 - 1), dir)
        }
        '[' => {
            check_ahead_vert(map, (next_pos.0, pos.1), dir)
                && check_ahead_vert(map, (pos.0 + dir.0, pos.1 + 1), dir)
        }
        _ => false,
    }
}


fn push_ahead_vert(
    map: &mut Vec<Vec<char>>,
    pos: (i32, i32),
    dir: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) {
    if visited.contains(&pos) {
        return; // Avoid revisiting the same position
    }

    visited.insert(pos);

    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

    // Check if the next position is inside the map bounds
    if !is_inside(map, next_pos) {
        return;
    }

    let next_char = map[next_pos.0 as usize][next_pos.1 as usize];

    match next_char {
        '.' => {
            map[next_pos.0 as usize][next_pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
            map[pos.0 as usize][pos.1 as usize] = '.';
        }
        ']' => {
            push_ahead_vert(map, (next_pos.0, pos.1 - 1), dir, visited); // Move left
            push_ahead_vert(map, (next_pos.0, pos.1), dir, visited); // Continue up
            map[next_pos.0 as usize][next_pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
            map[pos.0 as usize][pos.1 as usize] = '.';
        }
        '[' => {
            push_ahead_vert(map, (next_pos.0, pos.1), dir, visited); // Continue up
            push_ahead_vert(map, (next_pos.0, pos.1 + 1), dir, visited); // Move right
            map[next_pos.0 as usize][next_pos.1 as usize] = map[pos.0 as usize][pos.1 as usize];
            map[pos.0 as usize][pos.1 as usize] = '.';
        }
        _ => {
            println!("unexpected at {:?}", next_pos);
        }
    }
}

fn push_ahead_horiz(map: &mut Vec<Vec<char>>, start: (i32, i32), dir: (i32, i32), end: (i32, i32)) {
    let mut cur_pos = end;

    while cur_pos != start {
        let next_pos = (cur_pos.0 - dir.0, cur_pos.1 - dir.1);

        if !is_inside(map, next_pos) {
            break;
        }

        //println!(
        //"{:?} {:?}:  {} <=> {}",
        //cur_pos,
        //next_pos,
        //map[cur_pos.0 as usize][cur_pos.1 as usize],
        //map[next_pos.0 as usize][next_pos.1 as usize]
        //);

        map[cur_pos.0 as usize][cur_pos.1 as usize] = map[next_pos.0 as usize][next_pos.1 as usize];
        cur_pos = next_pos;
    }
    map[start.0 as usize][start.1 as usize] = '.';
}

fn move_robot_wider(map: &mut Vec<Vec<char>>, pos: &mut (i32, i32), dir: (i32, i32)) {
    let newpos = (pos.0 + dir.0, pos.1 + dir.1);
    if push_ahead(map, *pos, dir) {
        map[pos.0 as usize][pos.1 as usize] = '.';
        *pos = newpos;
        map[pos.0 as usize][pos.1 as usize] = '@';
    }
}

fn is_horizontal_move(dir: (i32, i32)) -> bool {
    dir.0 == 0
}

fn is_vertical_move(dir: (i32, i32)) -> bool {
    dir.1 == 0
}

fn robot_position(map: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = map.len();
    let cols = map[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }

    panic!("robot not found");
}

fn score(map: &Vec<Vec<char>>) -> i64 {
    let rows = map.len();
    let cols = map[0].len();
    let mut total: i64 = 0;

    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] == 'O' || map[i][j] == '[' {
                total += (100 * i + j) as i64;
            }
        }
    }
    total
}

fn robot_direction(robot: &char) -> (i32, i32) {
    match robot {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => panic!("Invalid guard character"),
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>())
    }
}

fn wider_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    map.iter()
        .map(|r| {
            let mut new_row = String::new();
            for c in r.iter() {
                let mapped = match *c {
                    'O' => "[]",
                    '.' => "..",
                    '#' => "##",
                    '@' => "@.",
                    _ => panic!("you should not be here bro!!!"),
                };
                new_row.push_str(mapped);
            }
            new_row.chars().collect()
        })
        .collect()
}

fn print_map_2(map: &Vec<Vec<char>>) {
    for row in map {
        let str_row = row.iter().collect::<String>();
        let wide_row = str_row
            .replace("O", "[]")
            .replace("#", "##")
            .replace("@", "@@")
            .replace(".", "..");
        println!("{}", wide_row);
    }
}

pub fn main() {
    let (map, directions) = read_input();

    {
        let mut map = map.clone();
        //part 01
        let mut pos = robot_position(&map);
        print_map(&map);
        for dir in &directions {
            move_robot(&mut map, &mut pos, robot_direction(dir));
            println!("\nMove {}", dir);
            print_map(&map);
        }
        println!("Score: {}", score(&map))
    }

    //really disliked this second part, kind of made everythnig uglier, eeek
    //part 01 was kinda cute
    //I think we can implement everything recursivelly to keep generic
    {
        //part02
        let mut map = wider_map(&map);
        let mut pos = robot_position(&map);
        print_map(&map);
        for dir in &directions {
            print!("\x1B[2J\x1B[1;1H");
            println!("\nMove {}", dir);
            move_robot_wider(&mut map, &mut pos, robot_direction(dir));
            print_map(&map);
            //sleep_ms(1000);
        }
        println!("Score: {}", score(&map))
    }
}
