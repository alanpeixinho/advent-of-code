use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

type Pos = (usize, usize);

fn read_input() -> (Pos, Pos, Pos, HashSet<Pos>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut blocked_nodes = HashSet::new();
    let mut size = (0, 0);
    for (row, line) in stdin().lines().enumerate() {
        size.0 = row + 1;
        for (col, c) in line.expect("Failed reading line").chars().enumerate() {
            size.1 = col + 1;
            match c {
                '#' => { blocked_nodes.insert((row, col)); }
                'S' => { start = (row, col); }
                'E' => { end = (row, col); }
                _ => {},
            }
        }
    }

    (start, end, size, blocked_nodes)
}

fn valid_pos(pos: (i32, i32), size: Pos) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && (pos.0 as usize) < size.0 && (pos.1 as usize) < size.1
}

fn bfs(
    start: Pos,
    end: Pos,
    size: Pos,
    blocked_nodes: &HashSet<Pos>,
    cheat_node: (usize, usize),
) -> i32 {
    let mut queue: VecDeque<(Pos, i32)> = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(start);

    while let Some((pos, level)) = queue.pop_front() {
        if pos == end {
            return level;
        };
        for off in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let neighbor = (
                (pos.0 as i32 + off.0 as i32),
                (pos.1 as i32 + off.1 as i32),
            );
            if valid_pos(neighbor, size) {
                let neighbor = (neighbor.0 as usize, neighbor.1 as usize);
                let cheat = neighbor == cheat_node;
                if !visited.contains(&neighbor)
                    && (cheat || !blocked_nodes.contains(&neighbor)) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, level + 1));
                }
            }
        }
    }
    0
}


fn cost_whole_path(
    start: Pos,
    end: Pos,
    size: Pos,
    blocked_nodes: &HashSet<Pos>
) -> HashMap<Pos, i32> {
    let mut queue: VecDeque<(Pos, i32)> = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(start);

    let mut all_level: HashMap<Pos, i32> = HashMap::new();
    all_level.insert(start, 0);

    while let Some((pos, level)) = queue.pop_front() {
        if pos == end {
            return all_level;
        };
        for off in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let neighbor = (
                (pos.0 as i32 + off.0 as i32),
                (pos.1 as i32 + off.1 as i32),
            );
            if valid_pos(neighbor, size) {
                let neighbor = (neighbor.0 as usize, neighbor.1 as usize);
                if !visited.contains(&neighbor)
                    && !blocked_nodes.contains(&neighbor)
                {
                    all_level.insert(neighbor, level + 1);
                    visited.insert(neighbor);
                    queue.push_back((neighbor, level + 1));
                }
            }
        }
    }
    unreachable!()
}

fn manhattan_distance(x1: (i32, i32), x2: (i32, i32)) -> i32 {
    i32::abs(x1.0 as i32 - x2.0 as i32) + i32::abs(x1.1 as i32 - x2.1 as i32)
}

fn cheat_neighborhood(max_cheat_dist: i32) -> Vec<(i32, i32)> {
    let mut cheat_neigh = Vec::new();
    for i in -max_cheat_dist..=max_cheat_dist {
        for j in -max_cheat_dist..=max_cheat_dist {
            if i == 0 && j == 0 { continue; }
            if manhattan_distance((i, j), (0, 0)) > max_cheat_dist { continue; }
            cheat_neigh.push((i, j));
        }
    }
    cheat_neigh
}

pub fn main() {
    let (start, end, size, blocked) = read_input();
    println!("{:?}", size);

    {
        let real_cost = bfs(start, end, size, &blocked, (usize::MAX, usize::MAX));
        let mut total_save = 0;
        for &node in blocked.iter() {
            let cheat_gain = real_cost - bfs(start, end, size, &blocked, node);
            if cheat_gain >= 100 {
                total_save += 1;
            }
        }
        println!("Part1 saved time: {:?}", total_save);
    }

    {
        let mut total_save = 0;
        let track_cost = cost_whole_path(start, end, size, &blocked);
        let cheat_neigh = cheat_neighborhood(20);
        for node in track_cost.keys() {
            let start_dist = track_cost.get(&node).unwrap();
            for (i, j) in cheat_neigh.iter() {
                let cur = (node.0 as i32 + i, node.1 as i32 + j);
                if valid_pos(cur, size)
                    && !blocked.contains(&(cur.0 as usize, cur.1 as usize)) {
                        let cheat_dist = manhattan_distance((node.0 as i32, node.1 as i32), cur);
                        let end_dist = track_cost.get(&(cur.0 as usize, cur.1 as usize)).unwrap();
                        let cheat_gain = end_dist - (start_dist + cheat_dist);
                    if cheat_gain >= 100 {
                        total_save += 1;
                    }
                }
            }
        }

        println!("Part 2 saved time {:?}", total_save);
    }
}
