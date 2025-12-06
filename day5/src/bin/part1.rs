#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let mut split = input.split("\n\n");
    let ranges_str = split.next().unwrap();
    let ingredients_str = split.next().unwrap();

    let ranges: Vec<_> = ranges_str.split('\n').map(to_range).collect();
    let ingredients: Vec<_> = ingredients_str
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    ingredients
        .into_iter()
        .filter(|ingredient| is_in_ranges(*ingredient, &ranges))
        .count()
}

fn to_range(line: &str) -> (usize, usize) {
    let mut split = line.split('-').map(|s| s.parse::<usize>().unwrap());
    (split.next().unwrap(), split.next().unwrap())
}

fn is_in_ranges(ingredient: usize, ranges: &Vec<(usize, usize)>) -> bool {
    ranges.iter().any(|range| is_in_range(ingredient, *range))
}

fn is_in_range(ingredient: usize, range: (usize, usize)) -> bool {
    range.0 <= ingredient && ingredient <= range.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let result = part1(input);
        assert_eq!(result, 3);
    }
}
