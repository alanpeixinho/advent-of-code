use std::io::stdin;

fn error(report: &Vec<i64>) -> Option<usize> {
    let sign = (report[1] - report[0]).signum();
    for i in 1..report.len() {
        let cur_diff = report[i] - report[i-1];
        if cur_diff.signum() != sign || (cur_diff.abs() > 3 || cur_diff.abs() <= 0) {
            return Some(i-1);
        }
    }
    None
}

fn soft_valid(report: &Vec<i64>, idx: usize) -> bool {
    for i in -1i64..=1i64 {
        let cur_idx = idx as i64 + i;
        let new_report: Vec<i64> = report.iter().enumerate()
            .filter(|(i, _)| *i as i64 != cur_idx).map(|(_, &x)| x).collect();
        if error(&new_report).is_none() {
            return true;
        }
    }
    false
}

pub fn main() {
    let mut correct = 0;
    let mut safe = 0;
    for line in stdin().lines() {
        let report: Vec<i64> = line.expect("fail on stdin").split(' ')
            .filter_map(|x| x.parse::<i64>().ok()).collect();

        let error = error(&report);

        match error {
            None => correct += 1,
            Some(idx) => safe += soft_valid(&report, idx) as i32
        }
    }

    println!("correct reports: {}", correct);
    println!("safe reports: {}", correct + safe);
}
