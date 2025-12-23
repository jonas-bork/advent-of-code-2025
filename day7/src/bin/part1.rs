#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/input.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(input: &str) -> u64 {
    let mut map = Map::parse(input);

    let (start_column, _) = map
        .lines
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, cell)| **cell == Cell::Start)
        .unwrap();

    start_beam(&mut map, 1, start_column)
}

fn start_beam(map: &mut Map, line_num: usize, column: usize) -> u64 {
    let Some(line) = map.lines.get_mut(line_num) else {
        return 0;
    };

    let Some(cell) = line.get_mut(column) else {
        return 0;
    };

    match *cell {
        Cell::Start => panic!("Start appeared in start_beam"),
        Cell::Empty => {
            *cell = Cell::Beam;
            start_beam(map, line_num + 1, column)
        }
        Cell::Splitter => {
            1 + start_beam(map, line_num, column - 1) + start_beam(map, line_num, column + 1)
        }
        Cell::Beam => 0,
    }
}

pub struct Map {
    pub lines: Vec<Vec<Cell>>,
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

        Self { lines }
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
        let result = part1(input);
        assert_eq!(result, 21);
    }
}
