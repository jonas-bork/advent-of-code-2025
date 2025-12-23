#![allow(dead_code)]

use std::{collections::HashMap, default};

fn main() {
    let file = include_str!("../data/input.txt");
    let result = part2(file);
    println!("{}", result);
}

fn part2(input: &str) -> u64 {
    let mut map = Map::parse(input);

    let (start_column, _) = map
        .lines
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, cell)| **cell == Cell::Start)
        .unwrap();

    1 + map.start_beam(1, start_column)
}

pub struct Map {
    pub lines: Vec<Vec<Cell>>,
    beam_results: HashMap<(usize, usize), u64>,
}

#[derive(PartialEq)]
pub enum Cell {
    Empty,
    Start,
    Splitter,
    Beam,
}

impl Map {
    pub fn parse(s: &str) -> Self {
        let lines = s
            .lines()
            .map(|line| line.chars().map(Cell::parse).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            lines,
            beam_results: HashMap::new(),
        }
    }

    pub fn start_beam(&mut self, line_num: usize, column: usize) -> u64 {
        if let Some(res) = self.beam_results.get(&(line_num, column)) {
            return *res;
        }

        let Some(line) = self.lines.get(line_num) else {
            return 0;
        };

        let Some(cell) = line.get(column) else {
            return 0;
        };

        let res = match *cell {
            Cell::Start => panic!("Start appeared in start_beam"),
            Cell::Empty => self.start_beam(line_num + 1, column),
            Cell::Splitter => {
                1 + self.start_beam(line_num, column - 1) + self.start_beam(line_num, column + 1)
            }
            Cell::Beam => panic!("Beam should never be encountered"),
        };

        self.beam_results.insert((line_num, column), res);
        res
    }
}

impl Cell {
    pub fn parse(c: char) -> Self {
        match c {
            'S' => Cell::Start,
            '.' => Cell::Empty,
            '^' => Cell::Splitter,
            _ => panic!("Unknown character: {c}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let result = part2(input);
        assert_eq!(result, 40);
    }
}
