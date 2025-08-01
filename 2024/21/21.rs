// disclosure, that was one boring problem to solve, had to resort to the hints and solutions at
// https://www.reddit.com/r/adventofcode/comments/1hj2odw/2024_day_21_solutions/ to get some ideas

use std::{collections::HashMap, io::stdin, iter::repeat};

type Pos = (i32, i32);

struct Keypad {
    char_map: HashMap<Pos, char>,
    pos_map: HashMap<char, Pos>
}

impl Keypad {
    fn new(buttons: &[(char, Pos)]) -> Self {
        let mut char_map = HashMap::with_capacity(buttons.len());
        let mut pos_map = HashMap::with_capacity(buttons.len());
        for &(c, pos) in buttons {
            char_map.insert(pos, c);
            pos_map.insert(c, pos);
        }
        Self { char_map , pos_map }
    }

    fn valid_pos(self: &Self, pos: Pos) -> bool {
        self.char_map.contains_key(&pos)
    }

    fn pos(self: &Self, c: char) -> Pos {
        self.pos_map.get(&c).copied()
            .expect(&format!("cant find code: {}", c))
    }

    fn _code(self: &Self, pos: Pos) -> Option<char> {
        self.char_map.get(&pos).copied()
    }
}

fn sub(a: Pos, b: Pos) -> Pos {
     (a.0 - b.0, a.1 - b.1)
}

fn read_input() -> Vec<String> {
    stdin()
        .lines()
        .map(|x| x.expect("cant read line").trim().to_owned())
        .collect()
}

fn numeric_code(code: &str) -> i32 {
    code.strip_suffix('A')
        .expect("cannot strip alpha")
        .parse()
        .unwrap_or(0)
}

fn precompute_path(keypad: &Keypad) -> HashMap<(char, char), String> {
    let mut map = HashMap::with_capacity(keypad.char_map.len().pow(2));
    for &from in keypad.char_map.values() {
        for &to in keypad.char_map.values() {
            let mut path = String::with_capacity(5);
            // the order is extremelly important,
            // we should aspire to go to the buttons farthest from 'A' first
            // < v ^ >
            let from_pos = keypad.pos(from);
            let to_pos = keypad.pos(to);
            let mut cur_pos = from_pos;
            while cur_pos != keypad.pos(to) {
                let diff = sub(to_pos, cur_pos);
                //println!("{:?} {:?} {:?}", keypad.pos(from), keypad.pos(to), cur_pos);
                if diff.1 < 0 { //left
                    cur_pos.1 += diff.1;
                    if keypad.valid_pos(cur_pos) {
                        path.extend(repeat('<').take(diff.1.abs() as usize));
                    } else {
                        cur_pos.1 -= diff.1;
                    }
                }
                if diff.0 > 0 { // down
                    cur_pos.0 += diff.0;
                    if keypad.valid_pos(cur_pos) {
                        path.extend(repeat('v').take(diff.0.abs() as usize));
                    } else {
                        cur_pos.0 -= diff.0;
                    }
                }
                if diff.0 < 0 { // up
                    cur_pos.0 += diff.0;
                    if keypad.valid_pos(cur_pos) {
                        path.extend(repeat('^').take(diff.0.abs() as usize));
                    } else {
                        cur_pos.0 -= diff.0;
                    }
                }
                if diff.1 > 0 { // right
                    cur_pos.1 += diff.1;
                    if keypad.valid_pos(cur_pos) {
                        path.extend(repeat('>').take(diff.1.abs() as usize));
                    } else {
                        cur_pos.1 -= diff.1;
                    }
                }
            }
            path.push('A');
            map.insert((from, to), path);
        }
    }
    map
}

fn dir_code_count(dir_paths: &HashMap<(char, char), String>, code: &str, level: i32,
    cache: &mut HashMap<(String, i32), i64>) -> i64 {

    if let Some(&count) = cache.get(&(code.into(), level)) {
        return count;
    }

    let mut count = 0;
    if level == 1 {
        count = dir_code_sequence(dir_paths, code).len() as i64;
    } else {
        let mut cur = 'A';
        for next in code.chars() {
            count += dir_code_count(dir_paths, &dir_paths[&(cur, next)], level - 1, cache);
            cur = next;
        }
    }

    cache.insert((code.into(), level), count);
    count
}

fn dir_code_sequence(dir_paths: &HashMap<(char, char), String>, code: &str) -> String {
    let mut cur = 'A';
    let mut path = String::new();
    for next in code.chars() {
        path.push_str(&dir_paths[&(cur, next)]);
        cur = next;
    }

    path
}


fn num_code_sequence(num_paths: &HashMap<(char, char), String>, code: &str) -> String {
    let mut cur = 'A';
    let mut path = String::new();
    for next in code.chars() {
        path.push_str(&num_paths[&(cur, next)]);
        cur = next;
    }

    path
}

fn numeric_keypad() -> Keypad {
    Keypad::new(&[
        ('7', (0, 0)), ('8', (0, 1)), ('9', (0, 2)),
        ('4', (1, 0)), ('5', (1, 1)), ('6', (1, 2)),
        ('1', (2, 0)), ('2', (2, 1)), ('3', (2, 2)),
                       ('0', (3, 1)), ('A', (3, 2)),
    ])
}

fn directional_keypad() -> Keypad {
    Keypad::new(&[
                       ('^', (0, 1)), ('A', (0, 2)),
        ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2)),
    ])
}

pub fn main() {
    let codes = read_input();

    let num_keypad = numeric_keypad();
    let num_paths = precompute_path(&num_keypad);

    let dir_keypad = directional_keypad();
    let dir_paths = precompute_path(&dir_keypad);

    {
        let mut complexity = 0;
        for code in codes.iter() {
            let mut tmp_code = num_code_sequence(&num_paths, &code);
            for _ in 0..2 {
                tmp_code = dir_code_sequence(&dir_paths, &tmp_code);
            }
            complexity += numeric_code(&code) * tmp_code.len() as i32;
        }
        println!("Part 1 complexity: {}", complexity);
    }
    {
        let mut complexity = 0;
        for code in codes.iter() {
            let tmp_code = num_code_sequence(&num_paths, &code);
            let count = dir_code_count(&dir_paths, &tmp_code, 25, &mut HashMap::new());
            complexity += numeric_code(&code) as i64 * count;
        }
        println!("Part 2 complexity: {}", complexity);
    }
}
