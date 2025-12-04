#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(ranges_str: &str) -> i64 {
    ranges_str
        .lines()
        .flat_map(|line| line.split(','))
        .flat_map(get_invalid_ids)
        .sum()
}

fn get_invalid_ids(range: &str) -> Vec<i64> {
    let mut split = range.split("-");
    let from = split.next().unwrap().parse::<i64>().unwrap();
    let to = split.next().unwrap().parse::<i64>().unwrap();

    return (from..to).filter(|n| is_invalid(*n)).collect();
}

fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    if s.len() % 2 == 1 {
        false
    } else {
        let break_point = s.len() / 2;
        let left = &s[0..break_point];
        let right = &s[break_point..];

        left == right
    }
}
