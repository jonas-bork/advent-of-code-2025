#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part2(file);
    println!("{}", result);
}

fn part2(input: &str) -> i64 {
    input.lines().map(|line| joltage(line, 12)).sum()
}

fn joltage(line: &str, length: usize) -> i64 {
    joltage_str(line, length).parse::<i64>().unwrap()
}

fn joltage_str(line: &str, length: usize) -> String {
    if length == 0 {
        return "".to_owned();
    }

    let nums: Vec<_> = line[0..line.len() - length + 1]
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();
    let max = nums.iter().max().unwrap();
    let (i, _) = nums.iter().enumerate().find(|(_, x)| *x == max).unwrap();

    line[i..i + 1].to_string() + joltage_str(&line[i + 1..], length - 1).as_str()
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
        let result = part2(input);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_single() {
        let input = "987654321111111";
        let result = part2(input);
        assert_eq!(result, 987654321111);
    }
}
