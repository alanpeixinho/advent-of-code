use std::io::stdin;

fn parse_range(s: &str) -> Option<(i64, i64)> {
    let str_range = s.split_once('-')?;
    Some((
        str_range.0.parse::<i64>().ok()?,
        str_range.1.parse::<i64>().ok()?,
    ))
}

fn read_ranges() -> Vec<(i64, i64)> {
    stdin()
        .lines()
        .map_while(|line| {
            let line = line.ok()?;
            let range_str = line.trim();
            if range_str.is_empty() {
                None
            } else {
                parse_range(range_str)
            }
        })
        .collect()
}

fn read_ids() -> Vec<i64> {
    stdin()
        .lines()
        .map(|line| {
            let id = line
                .expect("Invalid line")
                .parse::<i64>()
                .expect("Parsing error");
            id
        })
        .collect()
}

fn read_input() -> (Vec<(i64, i64)>, Vec<i64>) {
    (read_ranges(), read_ids())
}

fn in_range(n: i64, range: (i64, i64)) -> bool {
    n >= range.0 && n <= range.1
}

fn intersects(range1: (i64, i64), range2: (i64, i64)) -> bool {
    let st = i64::max(range1.0, range2.0);
    let en = i64::min(range1.1, range2.1);
    st <= en
}

fn union(range1: (i64, i64), range2: (i64, i64)) -> (i64, i64) {
    let st = i64::min(range1.0, range2.0);
    let en = i64::max(range1.1, range2.1);
    (st, en)
}

fn main() {
    let (ranges, ids) = read_input();

    let fresh_count = ids
        .into_iter()
        .filter(|&id| ranges.iter().any(|&r| in_range(id, r)))
        .count();

    println!("# of fresh ingredients: {}", fresh_count);

    let mut fused_ranges = Vec::with_capacity(ranges.len());

    for range in ranges.into_iter() {
        let union_range = fused_ranges
            .iter()
            .filter(|&&fr| intersects(range, fr))
            .fold(range, |acc, &r| union(acc, r));
        fused_ranges.retain(|&fr| !intersects(range, fr));
        fused_ranges.push(union_range);
    }

    let fresh_ids_count = fused_ranges
        .into_iter()
        .fold(0, |acc, fr| acc + fr.1 - fr.0 + 1);

    println!("# of all fresh ingredient ids possible: {}", fresh_ids_count);
}
