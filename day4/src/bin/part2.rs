#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let mut ss: Vec<_> = input
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .filter(|s| !s.is_empty())
        .collect();

    let height = ss.len();
    let width = ss[0].len();

    let pairs: Vec<_> = (0..height)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .collect();

    let mut total_removed = 0;

    loop {
        let removable: Vec<_> = pairs
            .iter()
            .filter(|(i, j)| ss[*i][*j] == '@')
            .filter(|coord| is_reachable(&ss, **coord))
            .collect();

        if removable.is_empty() {
            break;
        } else {
            total_removed += removable.len();
        }

        for (i, j) in removable {
            ss[*i][*j] = '.';
        }
    }

    total_removed
}

fn is_reachable(v: &Vec<Vec<char>>, coord: (usize, usize)) -> bool {
    let around: Vec<_> = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];

    around
        .into_iter()
        .map(|(i, j)| (i + coord.0, j + coord.1))
        .filter(|(i, j)| *i != 0 && *j != 0)
        .map(|(i, j)| (i - 1, j - 1))
        .filter(|(i, j)| *i != v.len() && *j != v[0].len())
        .filter(|(i, j)| v[*i][*j] == '@')
        .count()
        < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = part1(input);
        assert_eq!(result, 13);
    }
}
