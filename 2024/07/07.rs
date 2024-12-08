use std::{collections::HashMap, io::stdin};

extern crate itertools;

fn read_input() -> Vec<Vec<i64>> {
    let mut inputs = Vec::new();
    for line in stdin().lines() {
        let input = line.expect("fail");
        let nums: Vec<i64> = input
            .split(|c| c == ' ' || c == ':')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        inputs.push(nums);
    }
    inputs
}

fn ndigits(n: i64) -> u32 {
    (n as f32 + 0.1).log10().ceil() as u32
}

fn compute_equation(nums: &[i64], op: &Vec<char>) -> i64 {
    let mut total: i64 = nums[0];
    for i in 0..nums.len() - 1 {
        match op[i] {
            '*' => total *= nums[i + 1],
            '+' => total += nums[i + 1],
            '|' => total = total * i64::pow(10, ndigits(nums[i + 1])) + nums[i + 1],
            _ => panic!("invalid operand"),
        }
    }
    total
}

fn arrangements(elems: &[char], n: usize) -> Vec<Vec<char>> {
    let n_elem = elems.len();
    let n_arrangements = usize::pow(n_elem, n as u32);

    let mut all_arrangements = Vec::with_capacity(n_arrangements);

    for i in 0..n_arrangements{
        let mut arrangemnt = Vec::with_capacity(n);
        let mut number = i;
        for _ in 0..n {
            let elem_idx = number % n_elem;
            arrangemnt.push(elems[elem_idx as usize]);
            number /= n_elem;
        }
        all_arrangements.push(arrangemnt);
    }
    all_arrangements
}

pub fn main() {
    const ALL_OPERANDS: [char; 3] = ['*', '+', '|'];

    let mut total: i64 = 0;
    let mut cache_arrangements = HashMap::new();

    for input in read_input().iter() {
        let result = input[0];
        let nums = &input[1..];

        let all_arrangements = cache_arrangements
            .entry(nums.len() - 1)
            .or_insert_with(|| arrangements(&ALL_OPERANDS, nums.len() - 1));

        for operands in all_arrangements {
            if compute_equation(&nums, &operands) == result {
                total += result;
                break;
            }
        }
    }
    println!("total calibration: {:?}", total);
}
