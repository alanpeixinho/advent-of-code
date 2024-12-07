use std::io::stdin;


fn read_input() -> Vec<Vec<char>> {
    stdin().lines().map(|line| {
        line.expect("fail reading stdin").chars().collect::<Vec<char>>()
    }).collect()
}

fn print_field(field: &Vec<Vec<char>>) {
    for row in field {
        println!("{}", row.iter().cloned().collect::<String>());
    }
}

fn guard_direction(guard: char) -> (i32, i32) {
    match guard {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => panic!("Invalid guard character"),
    }
}

fn guard_position(field: &Vec<Vec<char>>) -> (i32, i32) {
    let valid_guard: [char; 4] = ['v', '^', '<', '>'];
    let rows = field.len();
    let cols = field[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if valid_guard.contains(&field[row][col]) {
                return (row as i32, col as i32);
            }
        }
    }
    panic!("there is no guard");
}

fn inside_field(field: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < field.len() as i32 && pos.1 >= 0 && pos.1 < field[0].len() as i32
}

fn rot90(dir: (i32, i32)) -> (i32, i32) {
    (dir.1, -dir.0)
}

fn update(field: &Vec<Vec<char>>, pos: &mut (i32, i32), dir: &mut (i32, i32)) {
    let mut newpos = (pos.0 + dir.0, pos.1 + dir.1);
    let mut newdir = (dir.0, dir.1);
    while inside_field(field, newpos) && field[newpos.0 as usize][newpos.1 as usize] == '#' {
        newdir = rot90(newdir);
        newpos = (pos.0 + newdir.0, pos.1 + newdir.1);
    }
    (pos.0, pos.1) = (newpos.0, newpos.1);
    (dir.0, dir.1) = (newdir.0, newdir.1);
}

fn count_coverage(field: &Vec<Vec<char>>) -> usize {
    field
        .iter()
        .map(|f| f.iter().filter(|&&c| c == 'X').count())
        .sum()
}

fn dir_symbol(dir: (i32, i32)) -> char {
    match dir {
        (-1, 0) => '^',
        (0, 1) => '>',
        (1, 0) => 'v',
        (0, -1) => '<',
        _ => panic!("invalid direction"),
    }
}

pub fn main() {
    let orig_field = read_input();

    {
        //part 01
        let mut field = orig_field.clone();
        let mut pos = guard_position(&field);
        let mut dir = guard_direction(field[pos.0 as usize][pos.1 as usize]);

        loop {
            field[pos.0 as usize][pos.1 as usize] = 'X';
            update(&mut field, &mut pos, &mut dir);
            if !inside_field(&field, pos) {
                break;
            }
        }

        print_field(&field);
        println!("step count: {}", count_coverage(&field));
    }

    {
        // part 02
        let mut loop_counter = 0;

        //that is almost certainly not the most efficient way to do this, but works nonetheless
        let rows = orig_field.len();
        let cols = orig_field[0].len();

        let orig_pos = guard_position(&orig_field);
        let orig_dir = guard_direction(orig_field[orig_pos.0 as usize][orig_pos.1 as usize]);

        for row in 0..rows {
            for col in 0..cols {
                if orig_field[row][col] != '.' {
                    continue;
                }

                let mut field = orig_field.clone();
                let mut pos = orig_pos;
                let mut dir = orig_dir;

                field[row][col] = '#';

                loop {
                    field[pos.0 as usize][pos.1 as usize] = dir_symbol(dir); //store the direction
                    update(&mut field, &mut pos, &mut dir);

                    if !inside_field(&field, pos) {
                        break;
                    }

                    if field[pos.0 as usize][pos.1 as usize] == dir_symbol(dir) {
                        loop_counter += 1;
                        break;
                    }
                }
            }
        }

        //print_field(&field);
        println!("loop count: {}", loop_counter);
    }
}
