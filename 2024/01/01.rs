use std::collections::HashMap;
use std::io;
use std::iter::zip;

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let input_lines = io::stdin().lines();

    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for (linenum, line) in input_lines.enumerate() {
        let linestr = line.expect("Failure during stdin read");
        let ids: Vec<i64> = linestr
            .split(' ')
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        if ids.len() >= 2 {
            list1.push(ids[0]);
            list2.push(ids[1]);
        } else {
            panic!("A line with less than 2 ids? Error at line {}", linenum);
        }
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

pub fn main() {
    let (list1, list2) = read_input();

    let total_distance: i64 = zip(list1.iter(), list2.iter())
        .map(|(x1, x2)| (x1 - x2).abs()).sum();

    let hist2: HashMap<i64, i64> = list2.iter()
        .fold(HashMap::new(), |mut h, &x| {
            *h.entry(x).or_insert(0) += 1;
            h
        });

    let similarity_score: i64 = list1.iter()
        .map(|x| x * hist2.get(x).unwrap_or(&0)).sum();

    println!("total distance: {}", total_distance);
    println!("similarity score: {}", similarity_score);
}
