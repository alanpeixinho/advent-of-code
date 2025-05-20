use std::{collections::{HashMap, HashSet}, io::stdin, iter::zip};

fn read_input() -> Vec<i64> {
    stdin()
        .lines()
        .map(|line| {
            line.expect("failure reading line")
                .parse()
                .expect("Failure parsing int")
        })
        .collect()
}

fn mix(x: i64, y: i64) -> i64 {
    x ^ y
}

fn prune(x: i64) -> i64 {
    x % 16777216
}

fn rule1(x: i64) -> i64 {
    prune(mix(x * 64, x))
}

fn rule2(x: i64) -> i64 {
    prune(mix(x / 32, x))
}

fn rule3(x: i64) -> i64 {
    prune(mix(x * 2048, x))
}

fn step(x: i64) -> i64 {
    rule3(rule2(rule1(x)))
}

fn steps(x: i64, nsteps: usize) -> Vec<i64> {
    let mut all_steps = Vec::with_capacity(nsteps + 1);
    all_steps.push(x);
    for i in 1..=nsteps {
        all_steps.push(step(all_steps[i - 1]));
    }
    all_steps
}

fn price(x: i64) -> i64 {
    x % 10
}

fn sequence4(all_steps: &Vec<Vec<i64>>) -> Vec<Vec<(i64, i64, i64, i64)>> {
    let prices: Vec<Vec<i64>> = all_steps
        .iter()
        .map(|x| x.iter().map(|&x| price(x)).collect())
        .collect();
    let nsecrets = all_steps.len();
    let nsteps = all_steps[0].len();
    let mut all_sequence4 = vec![vec![(0, 0, 0, 0); nsecrets]; nsteps];
    for step in 4..nsteps {
        for secret in 0..nsecrets {
            all_sequence4[step][secret] = (
                prices[secret][step - 3] - prices[secret][step - 4],
                prices[secret][step - 2] - prices[secret][step - 3],
                prices[secret][step - 1] - prices[secret][step - 2],
                prices[secret][step] - prices[secret][step - 1],
            );
        }
    }
    all_sequence4
}

fn buy_index(
    first_occurrences: &Vec<HashMap<(i64, i64, i64, i64), usize>>,
    sequence: (i64, i64, i64, i64),
) -> Vec<Option<usize>> {
    let nsecrets = first_occurrences.len();
    let mut indexes: Vec<Option<usize>> = vec![None; nsecrets];

    for secret in 0..nsecrets {
        indexes[secret] = first_occurrences[secret]
            .get(&sequence)
            .and_then(|x| Some(*x));
    }

    indexes
}

fn first_occurence_sequences(
    all_sequences: &Vec<Vec<(i64, i64, i64, i64)>>,
) -> Vec<HashMap<(i64, i64, i64, i64), usize>> {
    let nsteps = all_sequences.len();
    let nsecrets = all_sequences[0].len();

    let mut first_occurrences = vec![HashMap::with_capacity(nsteps); nsecrets];

    for step in 4..nsteps {
        for secret in 0..nsecrets {
            let sequence = all_sequences[step][secret];
            first_occurrences[secret].entry(sequence).or_insert(step);
        }
    }

    first_occurrences
}

fn sum_valid(all_prices: &Vec<Vec<i64>>, index: &Vec<Option<usize>>) -> i64 {
    zip(all_prices, index)
        .map(|(prices, i)| if i.is_some() { prices[i.unwrap()] } else { 0 })
        .sum()
}

pub fn main() {
    let nsteps: usize = 2000;
    let secrets = read_input();
    let all_steps: Vec<Vec<i64>> = secrets.iter().map(|&s| steps(s, nsteps)).collect();
    println!("Sum of 2000th step is {:?}", all_steps.iter().map(|s| s[2000]).sum::<i64>());

    let all_sequences = sequence4(&all_steps);
    let all_unique_sequences: HashSet<(i64, i64, i64, i64)> =
        all_sequences.iter().flatten().map(|x| *x).collect();

    let prices: Vec<Vec<i64>> = all_steps
        .iter()
        .map(|x| x.iter().map(|&x| price(x)).collect())
        .collect();

    let first_occurrences = first_occurence_sequences(&all_sequences);

    let max_cost = all_unique_sequences
        .iter()
        .map(|&sequence| sum_valid(&prices, &buy_index(&first_occurrences, sequence)))
        .max()
        .unwrap_or(0);

    println!("The cost of the best sequence is {:?}", max_cost);
}
