#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part2(file);
    println!("{}", result);
}

fn part2(input: &str) -> usize {
    let mut split = input.split("\n\n");
    let ranges_str = split.next().unwrap();

    let mut ranges: Vec<_> = ranges_str.split('\n').map(to_range).collect();
    ranges.sort_by_key(|range| range.0);

    let mut total = 0;
    let mut pointer = 0;
    for range in ranges {
        if range.0 > pointer {
            total += ids_in_range(range);
            pointer = range.1;
        } else if range.1 > pointer {
            total += range.1 - pointer;
            pointer = range.1;
        }
    }

    total
}

fn merge_ranges(ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let new_ranges: Vec<(usize, usize)> = vec![];

    for (i, range) in ranges.iter().enumerate() {
        for (j, other_range) in ranges[i + 1..].iter().enumerate() {
            todo!()
        }
    }

    new_ranges
}

fn ids_in_range(range: (usize, usize)) -> usize {
    range.1 - range.0 + 1
}

fn to_range(line: &str) -> (usize, usize) {
    let mut split = line.split('-').map(|s| s.parse::<usize>().unwrap());
    (split.next().unwrap(), split.next().unwrap())
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
        let result = part2(input);
        assert_eq!(result, 14);
    }
}
