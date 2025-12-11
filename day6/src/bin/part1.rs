#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/input.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(input: &str) -> i64 {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();

    transpose(lines)
        .iter()
        .map(|problem| {
            (
                problem[0..problem.len() - 1]
                    .into_iter()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
                Operation::from_str(problem.last().unwrap()),
            )
        })
        .map(|(problem, op)| op.apply(&problem))
        .sum()
}

fn transpose(v: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|inner| inner[i]).collect::<Vec<&str>>())
        .collect()
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, problem: &[i64]) -> i64 {
        match self {
            Operation::Add => problem.iter().sum(),
            Operation::Multiply => problem.iter().map(|x| *x).reduce(|x, y| x * y).unwrap(),
        }
    }

    fn from_str(op: &str) -> Self {
        match op {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => panic!("Unknown operation: {}", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let result = part1(input);
        assert_eq!(result, 4277556);
    }
}
