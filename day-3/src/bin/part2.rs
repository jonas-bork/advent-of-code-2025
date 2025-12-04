#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(input: &str) -> i64 {
    input.lines().map(joltage).sum()
}

fn joltage(line: &str) -> i64 {
    combinations(line)
        .map(|s| s.parse::<i64>().unwrap())
        .max()
        .unwrap()
}

fn combinations<'a>(line: &'a str) -> impl Iterator<Item = &str> {
    line.chars()
        .enumerate()
        .flat_map(|(i, x)| combinations(&'a line[i + 1..]).map(|y| format!("{x}{y}").as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let result = part1(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_1() {
        let input = "987654321111111";
        let result = part1(input);
        assert_eq!(result, 98);
    }
}
