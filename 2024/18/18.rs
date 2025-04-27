use std::{collections::{HashSet, VecDeque}, i32, io::stdin, iter::FromIterator};

const REGION_HEIGHT: i32 = 71;
const REGION_WIDTH: i32 = 71;

fn read_input() -> Vec<(i32, i32)>{
    fn parse_coord(word: &str) -> (i32, i32) {
        let coord: Vec<i32> = word.split(',')
            .map(|w| w.parse().expect("fail parsing int")).collect();
        assert!(coord.len() == 2);
        (coord[0], coord[1])
    }
    let lines = stdin().lines()
        .map(|line| line.expect("input read error"));
    let coords = lines.map(|s| parse_coord(&s)).collect();
    coords
}

fn valid_pos(pos: &(i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < REGION_WIDTH && pos.1 >= 0 && pos.1 < REGION_HEIGHT
}


fn bfs(pos: (i32, i32),
    blocked_nodes: &HashSet<(i32, i32)>) -> Option<i32> {
    let max_nodes = REGION_WIDTH * REGION_HEIGHT;
    let mut visited: HashSet<(i32, i32)> = HashSet::with_capacity(max_nodes as usize);
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::with_capacity(max_nodes as usize);

    queue.push_back((pos, 0));
    visited.insert(pos);
    while let Some((pos, level)) = queue.pop_front() {
        if pos == (REGION_WIDTH - 1, REGION_HEIGHT - 1) {
            return Some(level);
        }
        for off in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let neighbor = (pos.0 + off.0, pos.1 + off.1);
            if valid_pos(&neighbor) &&
                !visited.contains(&neighbor) &&
                !blocked_nodes.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, level + 1));
            }
        }
    }
    None
}

fn main() {
    let input = read_input();

    {
        let blocked_nodes = HashSet::from_iter(input.iter().take(1024).cloned());
        let target = bfs((0, 0), &blocked_nodes);
        println!("Cost to travess after first 1024 bytes corrupted: {}", target.unwrap());
    }

    for i in 0..input.len() {
        let blocked_nodes = HashSet::from_iter(input.iter().take(i).cloned());
        let target = bfs((0, 0), &blocked_nodes);
        if target == None {
            println!("Last corrupted byte before impossible travess: {:?}", input[i-1]);
            break;
        }
    }
}
