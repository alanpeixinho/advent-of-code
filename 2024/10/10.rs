use std::{collections::VecDeque, io::stdin};

type Map = Vec<Vec<i32>>;
type Position = (usize, usize);

fn new_map(rows: usize, cols: usize) -> Map {
    vec![vec![0; cols]; rows]
}

fn read_input() -> Map {
    let lines = stdin().lines();
    let mut data: Map = Map::new();
    for line in lines {
        let row: Vec<i32> = line.expect("error on read")
            .chars().map(|c| c.to_digit(10).expect("invalid value (0-9)") as i32)
            .collect();
        data.push(row);
    }
    data
}

fn map_dims(map: &Map) -> Position {
    (map.len(), map[0].len())
}

fn start_pos(data: &Map) -> Vec<Position> {
    let start_level = 9;
    let mut pos = Vec::new();
    let (rows, cols) = map_dims(data);
    for row in 0..rows {
        for col in 0..cols {
            if data[row][col] == start_level {
                pos.push((row, col));
            }
        }
    }
    pos
}

fn is_valid_pos(size: &Position, position: (i32, i32)) -> bool {
    position.0 >= 0 && position.1 >= 0 &&
    position.0 < size.0 as i32 && position.1 < size.1 as i32
}

fn are_neighbors(map: &Map, from: &Position, to: &Position) -> bool {
    map[from.0][from.1] == (map[to.0][to.1] + 1)
}

fn dfs(map: &Map, start_pos: &Position) -> i32 {
    let (rows, cols) = map_dims(map);
    let mut visited = new_map(rows, cols);
    dfs_recursive(map, &mut visited, start_pos)
}

fn dfs_recursive(map: &Map, visited: &mut Map, start_pos: &Position) -> i32 {
    let (rows, cols) = map_dims(map);
    //println!("dfs: {start_pos:?}");

    visited[start_pos.0][start_pos.1] = 1;
    if map[start_pos.0][start_pos.1] == 0 {
        return 1;
    }

    let mut score = 0;
    for (i, j) in [ (-1, 0), (1, 0), (0, -1), (0, 1) ] {
        let (nrow, ncol) = (start_pos.0 as i32 + i, start_pos.1 as i32 + j);
        if is_valid_pos(&(rows, cols), (nrow, ncol)) {
            let (nrow, ncol) = (nrow as usize, ncol as usize);
            if are_neighbors(&map, &start_pos, &(nrow, ncol)) && visited[nrow][ncol] <= 0 {
                score += dfs_recursive(map, visited, &(nrow, ncol));
            }
        }
    }

    visited[start_pos.0][start_pos.1] = 0;
    score
}

fn bfs(map: &Map, start_pos: &Position) -> i32 {
    let (rows, cols) = map_dims(map);

    let mut all_paths = Vec::new();
    let mut queue = VecDeque::with_capacity(rows * cols);
    let mut cur_path = VecDeque::with_capacity(rows * cols);

    queue.push_back(*start_pos);
    cur_path.push_back(*start_pos);

    while let Some(pos) = queue.pop_back() {
        cur_path.push_back(pos);
        for (i, j) in [ (-1, 0), (1, 0), (0, -1), (0, 1) ] {
            let (nrow, ncol) = (pos.0 as i32 + i, pos.1 as i32 + j);
            if is_valid_pos(&(rows, cols), (nrow, ncol)) {
                let (nrow, ncol) = (nrow as usize, ncol as usize);
                if are_neighbors(&map, &pos, &(nrow, ncol)) {
                    queue.push_back((nrow, ncol));
                    if map[nrow][ncol] == 0 {
                        all_paths.push(cur_path.clone());
                    }
                }
            }
        }
        cur_path.pop_back();
    }

    all_paths.len() as i32
}

pub fn main() {
    let map = read_input();
    let positions = start_pos(&map);
    let sum_score_1: i32 = positions.iter().map(|&pos| dfs(&map, &pos)).sum();
    let sum_score_2: i32 = positions.iter().map(|&pos| bfs(&map, &pos)).sum();
    println!("score1: {sum_score_1:?} score2: {sum_score_2:?}");
}
