#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part2(file);
    println!("{}", result);
}

fn part2(ranges_str: &str) -> i64 {
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

    return (from..to + 1).filter(|n| is_invalid(*n)).collect();
}

fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    (1..s.len())
        .map(|x| s[0..x].to_string())
        .any(|x| is_repeated(&s, x))
}

fn is_repeated(s: &String, x: String) -> bool {
    if s.len() % x.len() != 0 {
        return false;
    }
    let mut start = 0;
    loop {
        let end = start + x.len();
        if end > s.len() {
            return true;
        }
        if s[start..end] != x {
            return false;
        }
        start = end
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated() {
        let result = get_invalid_ids("11-22");
        assert_eq!(result, vec![11, 22])
    }

    #[test]
    fn test_repeated2() {
        let result = get_invalid_ids("95-115");
        assert_eq!(result, vec![99, 111])
    }

    #[test]
    fn test_repeated3() {
        let result = get_invalid_ids("998-1012");
        assert_eq!(result, vec![999, 1010])
    }
}
