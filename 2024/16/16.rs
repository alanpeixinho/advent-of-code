use std::{
    cmp::Reverse, collections::{BinaryHeap, HashSet, LinkedList}, f32::INFINITY, hash::Hash, i32::{self, MAX}, io::stdin, iter::{self, zip}, thread::sleep_ms, usize
};

type Map = Vec<Vec<char>>;
type Edge = ((usize, usize), (usize, usize));
type CostMap = Vec<Vec<i32>>;
type PredMap = Vec<Vec<Option<(usize, usize)>>>;
type PathMap = Vec<Vec<Vec<(usize, usize)>>>;

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

fn print_map(map: &Map) {
    for (i, row) in map.iter().enumerate() {
        println!("{:02} - {}", i, row.iter().collect::<String>())
    }
}

fn print_cost(cost: &CostMap) {
    for row in cost {
        for val in row {
            if *val == MAX {
                print!("##### ");
            } else {
                print!("{:05} ", val);
            }
        }
        println!();
    }
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

fn path_map(map: &Map) -> PathMap {
    let (rows, cols) = dims(map);
    let path = vec![vec![Vec::new(); cols]; rows];
    path
}

fn pred_map(map: &Map) -> PredMap {
    let (rows, cols) = dims(map);
    let prev = vec![vec![None; cols]; rows];
    prev
}

fn neighbors(map: &Map, pos: (usize, usize), cur_dir: (i32, i32)) -> Vec<(usize, usize)> {
    let (rows, cols) = dims(map);

    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    // order of less cost
    //let adj = [cur_dir, (cur_dir.1, cur_dir.0), (-cur_dir.1, -cur_dir.0), (-cur_dir.0, -cur_dir.1)];

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

fn cheapest_neighbors(cost: &CostMap, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (rows, cols) = dims(cost);

    let mut neighbors = Vec::new();
    let mut min_cost: i32 = MAX;

    let adj = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let p_cost = cost[pos.0][pos.1];

    for (i, j) in adj {
        let ni = pos.0 as i64 + i;
        let nj = pos.1 as i64 + j;

        if ni < 0 || ni >= rows as i64 || nj < 0 || nj >= cols as i64 {
            continue;
        }

        let n_cost = cost[ni as usize][nj as usize];

        if n_cost < p_cost {
            //cannot go back
            continue;
        }

        if n_cost < min_cost {
            min_cost = n_cost;
            neighbors = vec![(ni as usize, nj as usize)];
        } else if n_cost == min_cost {
            neighbors.push((ni as usize, nj as usize));
        }
    }

    neighbors
}

fn parent_list(cost: &CostMap, p: (usize, usize)) -> Vec<(usize, usize)> {
    let mut min_cost_idx = Vec::new();
    let mut min_cost = MAX;
    let adj = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let (rows, cols) = dims(cost);

    for (i, n) in adj.iter().enumerate() {
        let ni = p.0 as i32 + n.0;
        let nj = p.1 as i32 + n.1;
        if ni < 0 || ni >= rows as i32 || nj < 0 || nj >= cols as i32 {
            continue;
        }
        let c = cost[ni as usize][nj as usize];
        if c < min_cost {
            min_cost = c;
            min_cost_idx = vec![i];
        } else if c == min_cost {
            min_cost_idx.push(i);
        }
    }

    min_cost_idx
        .iter()
        .map(|i| {
            (
                (p.0 as i32 + adj[*i].0) as usize,
                (p.1 as i32 + adj[*i].1) as usize,
            )
        })
        .collect()
}

fn shortest_path_nodes_recursion(
    prev: &PathMap,
    node: (usize, usize),
    path: &mut HashSet<(usize, usize)>,
) {
    path.insert(node);
    for &predecessor in prev[node.0][node.1].iter() {
        shortest_path_nodes_recursion(prev, predecessor, path);
    }
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


fn dfs_rec(
    map: &mut Map,
    cost: &CostMap,
    best_cost: i32,
    start_pos: (usize, usize),
    start_dir: (i32, i32),
    end_pos: (usize, usize),
    path: &mut Vec<(usize, usize)>
) -> bool {

    if cost[end_pos.0][end_pos.1] > best_cost {
        return false;
    }

    if start_pos == end_pos {
        path.push(end_pos);
        //println!("{:?} {:?}", end_pos, path.len());
        return true;
    }

    let map_val = map[end_pos.0][end_pos.1];

    map[end_pos.0][end_pos.1] = '*';

    //let mut all_visited = HashSet::new();
    let mut visited = false;
    //println!("****** {:?} -> {:?}", start_pos, neighbors(map, end_pos, start_dir));
    for neighbor_pos in neighbors(map, end_pos, start_dir) {
        if map[neighbor_pos.0][neighbor_pos.1] != '*' {
            //println!("{:?} {:?} {:?} {:?}", start_pos, neighbor_pos, best_cost, cost[neighbor_pos.0][neighbor_pos.1]);
            let neigh_dir = direction(end_pos, neighbor_pos);
                //if cost[end_pos.0][end_pos.1] > cost[neighbor_pos.0][neighbor_pos.1] {
            if true {
                let neigh_visited = dfs_rec(
                    map,
                    cost,
                    best_cost,
                    start_pos,
                    neigh_dir,
                    neighbor_pos,
                    path
                );


                visited = visited || neigh_visited;
            }
            //for v in visited {
                //all_visited.insert(v);
            //}

        }
    }

    map[end_pos.0][end_pos.1] = map_val;

    if visited {
        //println!("{:?} {:?}", end_pos, path.len());
        path.push(end_pos);
        //all_visited.insert(start_pos);
        return true;
    }

    false
}

fn dfs_rec_2(
    map: &mut Map,
    best_cost: i32,
    cur_cost: i32,
    start_pos: (usize, usize),
    start_dir: (i32, i32),
    end_pos: (usize, usize),
    path: &mut Vec<(usize, usize)>
) -> bool {

    if cur_cost > best_cost {
        //println!("ahhhhhhh");
        return false;
    }

    if start_pos == end_pos {
        if cur_cost != best_cost {
            panic!("---- {} {}", cur_cost, best_cost);
        }
        path.push(start_pos);
        //println!("{:?} {:?}", start_pos, path.len());
        return true;
    }

    let map_val = map[start_pos.0][start_pos.1];

    map[start_pos.0][start_pos.1] = '*';

    //let mut all_visited = HashSet::new();
    let mut visited = false;
    for neighbor_pos in neighbors(map, start_pos, start_dir) {
        if map[neighbor_pos.0][neighbor_pos.1] != '*' {
            let neigh_dir = direction(start_pos, neighbor_pos);
            let neigh_visited = dfs_rec_2(
                map,
                best_cost,
                cur_cost + cost_edge(start_dir, neigh_dir),
                neighbor_pos,
                neigh_dir,
                end_pos,
                path
            );

            visited = visited || neigh_visited;

            //for v in visited {
                //all_visited.insert(v);
            //}

        }
    }

    map[start_pos.0][start_pos.1] = map_val;

    if visited {
        println!("{:?} {:?}", start_pos, path.len());
        path.push(start_pos);
        //all_visited.insert(start_pos);
        return true;
    }

    false
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
    //let mut paths = path_map(map);
    let mut prev = pred_map(map);
    let mut queue = BinaryHeap::new();
    let mut cur_pos;

    queue.push((Reverse(0), start_pos));

    while !queue.is_empty() {
        //println!("queue loop");
        (_, cur_pos) = queue.pop().unwrap();
        if map[cur_pos.0][cur_pos.1] == '*' {
            continue;
        } // visited
        let cur_cost = cost[cur_pos.0][cur_pos.1];
        //println!(
            //"min neighbor of {:?} {:?}",
            //cur_pos,
            //min_cost_direction(&cost, cur_pos)
        //);
        let cur_dir = min_cost_direction(&cost, cur_pos);
        //println!("{:?} neighbors: {:?}", cur_pos, neighbors(map, cur_pos, cur_dir));
        for n in neighbors(map, cur_pos, cur_dir) {
            if blocked_edges.contains(&(cur_pos, n)) { continue; }
            //println!("dirs: {:?} {:?}", cur_dir, direction(cur_pos, n));
            let cost_neighbor = cur_cost + cost_edge(cur_dir, direction(cur_pos, n));
            //println!(
                //"{:?} => Cost: {} {}",
                //cur_pos, cost_neighbor, cost[n.0][n.1]
            //);
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
    let K = 100;

    let (cost, pred) = djikstra(&mut map.clone(), start_pos, &blocked_edges);
    let min_cost = cost[end_pos.0][end_pos.1];

    let mut A = vec![path_to(&pred, start_pos, end_pos)];
    let mut B = HashSet::new();

    let mut i = 1;
    for k in 0..K {
        //println!("trying {} ...", i);
        for i in 0..(A[k].len()-2) {
            //println!("i = {}", i);
            let pivot_pos = A[k][i];
            let root_path = &A[k][..i];
            //println!("===================\n");
            for  p in A.iter() {
                //println!("root path: {:?}", root_path);
                //println!("root path: {:?}", p);
                if p.len() >= i && root_path.eq(&p[..i]) {
                    blocked_edges.insert((p[i], p[i+1]));
                }
            }

            for &pos in root_path {
                if pos != pivot_pos {
                    map[pos.0][pos.1] = '#'; //remove node
                }
            }

            //println!("compute dijkstra for {:?}", pivot_pos);
            let mut pivot_map = map.clone();


            //print_map(&pivot_map);
            let (pivot_cost, pivot_pred) = djikstra(&mut pivot_map, pivot_pos, &blocked_edges);
            let pivot_path = path_to(&pivot_pred, pivot_pos, end_pos);

            //println!("pivot_path: {:?}", pivot_path);

            //print_map(&pivot_map);
            //print_cost(&pivot_cost);

            if !pivot_path.is_empty() {
                let total_path: Vec<(usize, usize)> = root_path.iter().chain(pivot_path.iter()).cloned().collect();

                // do not exist in the original algorithm, but here, we are looking only for the same
                // cost paths
                B.insert(total_path);

            }
            //println!("B size = {}", B.len());

            blocked_edges.clear();
            for pos in root_path {
                map[pos.0][pos.1] = '.';
            }

        }

        if B.is_empty() { break; }
        //println!("tried and got {} paths and {} blocked edges", B.len(), blocked_edges.len());

        let (costnewpath, newpath) = pop_min_path(&mut B);
        //println!("new path is {:?} with cost: {:?}", newpath, costnewpath);

        if costnewpath <= min_cost &&  newpath.last() == Some(&end_pos) {
            A.push(newpath);
        } else {
            break; // if the last shortest path is greater than the min (in this problem) we can
                   // stop searching
        }

        i += 1;
    }
    A
}

fn pop_min_path(B: &mut HashSet<Vec<(usize, usize)>>) -> (i32, Vec<(usize, usize)>) {
    let mut min_cost = MAX;
    let mut min_path = None;                                ;
    for p in B.iter() {
        let cost = path_cost(p);
        if  cost < min_cost {
            min_cost = cost;
            min_path = Some(p.clone());
            //println!("found new path: {:?}", p);
        }
    }
    if let Some(p) = min_path {
        B.remove(&p);
        return (min_cost, p);
    }
    (MAX, vec![])
}

fn count_all_possible_nodes(all_paths:& Vec<Vec<(usize, usize)>>) -> usize {
    let mut unique = HashSet::new();
    for path in all_paths.iter() {
        for node in path.iter() {
            unique.insert(node);
        }
    }
    unique.len()
}

pub fn main() {
    let mut map = read_input();
    let start_pos = find_start(&map);
    let end_pos = find_end(&map);
    //print_map(&map);
    println!("{:?} {:?}", find_start(&map), find_end(&map));
    let mut blocked_edges = HashSet::new();
    blocked_edges.insert(((11,4), (11,5)));
    let (cost, pred) = djikstra(&mut map.clone(), start_pos, &blocked_edges);

    println!("{:?}", path_to(&pred, start_pos, end_pos));
    println!("{:?}", cost[end_pos.0][end_pos.1]);


    //let (inv_cost, _) = djikstra(&mut map.clone(), end_pos);

    let (rows, cols) = dims(&map);

    //for r in 0..rows {
        //for c in 0..cols {
            //if cost[r][c] > cost[end_pos.0][end_pos.1] {
                //map[r][c] = '#';
            //}
        //}
    //}

    //let cost = bfs(&mut map.clone());
    //print_map(&map);
    //print_cost(&cost);
    println!("Lowest cost: {}", cost[end_pos.0][end_pos.1]);

    //print_map(&map);

    //print_cost(&cost);

    //let mut path = Vec::with_capacity(20000);
    //let set = dfs_rec_2(&mut map.clone(),  cost[end_pos.0][end_pos.1], 0, start_pos, (0, 1), end_pos, &mut path);
    ////dfs_rec(&mut map.clone(), &cost, cost[end_pos.0][end_pos.1], start_pos, (0, 1), end_pos, &mut path);

    ////println!("{:?}", set);

    //print_map(&map);

    //let p1 = path.iter().collect::<HashSet<_>>();
    //let p2 = pred[end_pos.0][end_pos.1].iter().collect::<HashSet<_>>();
    ////println!("score: {:?}", set.len());
    //println!("wrong: {:?}", p1);

    //println!("{:?}", p2);

    //println!("diff = {:?}", p1.difference(&p2));

    //for node in path_to(&pred, start_pos, end_pos) {
        //map[node.0][node.1] = 'O';
    //}

    //print_map(&map);

    //print_cost(&cost);

    let all_paths = topk_shortest_paths(&mut map.clone(), start_pos, end_pos);


    for path in all_paths.iter() {
        for node in path {
            map[node.0][node.1] = 'O';
        }
    }

    for path in all_paths.iter() {
        println!("{:?}", path);
    }

    print_map(&map);

    println!("count available: {}", count_all_possible_nodes(&all_paths));

    //for s in set {
        //if !path.contains(&s) {
            //println!("sai paiacao {:?}", s);
        //}
    //}

}
