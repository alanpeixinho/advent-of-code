use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::i32::MAX;
use std::io::stdin;

type Map = Vec<Vec<char>>;
type Edge = ((usize, usize), (usize, usize));
type CostMap = Vec<Vec<i32>>;
type PredMap = Vec<Vec<Option<(usize, usize)>>>;

fn dims<T>(map: &Vec<Vec<T>>) -> (usize, usize) {
    let rows = map.len();
    let cols = map.get(0).map_or(0, |r| r.len());
    (rows, cols)
}

fn read_input() -> Map {
    let mut map = Map::new();
    for line in stdin().lines() {
        map.push(line.expect("error reading").chars().collect());
    }
    map
}

fn find_position(map: &Map, symbol: char) -> (usize, usize) {
    let (rows, cols) = dims(map);
    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == symbol {
                return (row, col);
            }
        }
    }
    panic!("Could not find the guy");
}

fn find_start(map: &Map) -> (usize, usize) {
    find_position(map, 'S')
}

fn find_end(map: &Map) -> (usize, usize) {
    find_position(map, 'E')
}

fn cost_map(map: &Map, start_pos: (usize, usize)) -> CostMap {
    let (rows, cols) = dims(map);
    let mut cost = vec![vec![MAX; cols]; rows];
    cost[start_pos.0][start_pos.1] = 0;
    cost
}

fn pred_map(map: &Map) -> PredMap {
    let (rows, cols) = dims(map);
    let prev = vec![vec![None; cols]; rows];
    prev
}

fn neighbors(map: &Map, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (rows, cols) = dims(map);

    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    let adj = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for (i, j) in adj {
        let ni = pos.0 as i32 + i;
        let nj = pos.1 as i32 + j;

        if ni < 0 || ni >= rows as i32 || nj < 0 || nj >= cols as i32 {
            continue;
        }
        match map[ni as usize][nj as usize] {
            '.' | 'E' | 'S' => {
                neighbors.push((ni as usize, nj as usize));
            }
            _ => (),
        }
    }

    neighbors
}

fn min_cost_direction(cost: &CostMap, p: (usize, usize)) -> (i32, i32) {
    let mut min_cost_idx = 0usize;
    let mut min_cost = MAX;
    let adj = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let (rows, cols) = dims(cost);

    for (i, n) in adj.iter().enumerate() {
        let ni = p.0 as i32 + n.0;
        let nj = p.1 as i32 + n.1;
        if ni < 0 || ni >= rows as i32 || nj < 0 || nj >= cols as i32 {
            continue;
        }

        if cost[ni as usize][nj as usize] < min_cost {
            min_cost = cost[ni as usize][nj as usize];
            min_cost_idx = i;
        }
    }

    if min_cost == MAX {
        return (0, 1);
    }; // default to east

    let min_cost_neighbor = adj[min_cost_idx];
    (-min_cost_neighbor.0, -min_cost_neighbor.1)
}

fn direction(p1: (usize, usize), p2: (usize, usize)) -> (i32, i32) {
    let dir = (p2.0 as i32 - p1.0 as i32, p2.1 as i32 - p1.1 as i32);
    if dir == (0, 0) || dir.0.abs() >= 2 || dir.1.abs() >= 2 {
        panic!("something went wrong here: {:?} {:?}", p1, p2);
    }
    dir
}

fn cost_edge(cur_dir: (i32, i32), neigh_dir: (i32, i32)) -> i32 {
    match (cur_dir, neigh_dir) {
        ((x1, y1), (x2, y2)) if x1 == x2 && y1 == y2 => 1,
        ((x1, y1), (x2, y2)) if (x1 - x2).abs() >= 2 || (y1 - y2).abs() >= 2 => 2001,
        _ => 1001,
    }
}

fn path_cost(path: &Vec<(usize, usize)>) -> i32 {
    let mut cost = 0;
    let mut cur_dir = (0, 1);
    for i in 1..path.len() {
        let prev_dir = cur_dir;
        cur_dir = direction(path[i-1], path[i]);
        cost += cost_edge(prev_dir, cur_dir);
    }
    cost
}

fn path_to(prev: &PredMap, start_pos: (usize, usize), end_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut cur_pos = end_pos;

    loop {
        path.push(cur_pos);
        if cur_pos == start_pos { break; }
        match prev[cur_pos.0][cur_pos.1] {
            Some(x) => { cur_pos = x; },
            None => { return vec![]; }
        }
    }

    path.reverse();
    path
}

fn djikstra(map: &mut Map, start_pos: (usize, usize), blocked_edges: &HashSet<Edge>) -> (CostMap, PredMap) {
    let mut cost = cost_map(map, start_pos);
    let mut prev = pred_map(map);
    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0), start_pos));

    while !queue.is_empty() {
        //println!("queue loop");
        let (_, cur_pos) = queue.pop().unwrap();
        if map[cur_pos.0][cur_pos.1] == '*' {
            continue;
        } // visited
        let cur_cost = cost[cur_pos.0][cur_pos.1];
        let cur_dir = min_cost_direction(&cost, cur_pos);
        for n in neighbors(map, cur_pos) {
            if blocked_edges.contains(&(cur_pos, n)) { continue; }
            let cost_neighbor = cur_cost + cost_edge(cur_dir, direction(cur_pos, n));
            if cost_neighbor < cost[n.0][n.1] {
                cost[n.0][n.1] = cost_neighbor;
                prev[n.0][n.1] = Some(cur_pos);
                queue.push((Reverse(cost_neighbor), n));
            }
        }
        map[cur_pos.0][cur_pos.1] = '*'; // visited
    }
    (cost, prev)
}

fn topk_shortest_paths(map: &mut Map, start_pos: (usize, usize), end_pos: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut blocked_edges = HashSet::new();

    //k shortest paths
    let k = 1024;

    let (cost, pred) = djikstra(&mut map.clone(), start_pos, &blocked_edges);
    let min_cost = cost[end_pos.0][end_pos.1];

    let mut a = vec![path_to(&pred, start_pos, end_pos)];
    let mut b = HashSet::new();

    for k in 0..k {
        for i in 0..(a[k].len()-2) {
            let pivot_pos = a[k][i];
            let root_path = &a[k][..i];
            for  p in a.iter() {
                if p.len() >= i && root_path.eq(&p[..i]) {
                    blocked_edges.insert((p[i], p[i+1]));
                }
            }

            for &pos in root_path {
                if pos != pivot_pos {
                    map[pos.0][pos.1] = '#'; //remove node
                }
            }

            let mut pivot_map = map.clone();

            let (_, pivot_pred) = djikstra(&mut pivot_map, pivot_pos, &blocked_edges);
            let pivot_path = path_to(&pivot_pred, pivot_pos, end_pos);

            if !pivot_path.is_empty() {
                let total_path: Vec<(usize, usize)> = root_path.iter().chain(pivot_path.iter()).cloned().collect();

                // do not exist in the original algorithm, but here, we are looking only for the same
                // cost paths
                b.insert(total_path);
            }

            blocked_edges.clear();
            for pos in root_path {
                map[pos.0][pos.1] = '.';
            }

        }

        let (costnewpath, newpath) = pop_min_path(&mut b);
        //println!("new path is {:?} with cost: {:?}", newpath, costnewpath);

        if costnewpath <= min_cost &&  newpath.last() == Some(&end_pos) {
            a.push(newpath);
        } else {
            break; // if the last shortest path is greater than the min (in this specific problem)
                   // we can stop searching
        }
    }
    a
}

fn pop_min_path(paths: &mut HashSet<Vec<(usize, usize)>>) -> (i32, Vec<(usize, usize)>) {
    let mut min_cost = MAX;
    let mut min_path = None;
    for p in paths.iter() {
        let cost = path_cost(p);
        if  cost < min_cost {
            min_cost = cost;
            min_path = Some(p.clone());
        }
    }
    if let Some(p) = min_path {
        paths.remove(&p);
        return (min_cost, p);
    }
    (MAX, vec![])
}

fn count_all_possible_nodes(all_paths:& Vec<Vec<(usize, usize)>>) -> usize {
    let mut unique = HashSet::new();
    for node in all_paths.iter().flatten() {
        unique.insert(node);
    }
    unique.len()
}

pub fn main() {
    let mut map = read_input();
    let start_pos = find_start(&map);
    let end_pos = find_end(&map);

    let blocked_edges = HashSet::new();
    let (cost, _) = djikstra(&mut map.clone(), start_pos, &blocked_edges);

    println!("Lowest cost: {}", cost[end_pos.0][end_pos.1]);

    let all_paths = topk_shortest_paths(&mut map.clone(), start_pos, end_pos);
    for path in all_paths.iter() {
        for node in path {
            map[node.0][node.1] = 'O';
        }
    }

    println!("count available: {}", count_all_possible_nodes(&all_paths));
}
