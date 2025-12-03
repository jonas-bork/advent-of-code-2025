#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part1(file);
    println!("{}", result);
}

enum Direction {
    Left,
    Right,
}

struct Turn {
    direction: Direction,
    amount: i32,
}

fn part1(input: &str) -> i64 {
    let turns: Vec<_> = input.lines().map(parse_turn).collect();

    let mut password = 0;
    let mut dial = 50;
    for turn in turns {
        dial = apply_turn(dial, turn);
        if dial == 0 {
            password += 1;
        }
    }

    password
}

fn apply_turn(n: i32, turn: Turn) -> i32 {
    let result = match turn.direction {
        Direction::Left => n - turn.amount,
        Direction::Right => n + turn.amount,
    };

    result % 100
}

fn parse_turn(s: &str) -> Turn {
    let dir_str = s.get(0..1).unwrap();
    let dir = if dir_str == "L" {
        Direction::Left
    } else {
        Direction::Right
    };

    let amount = s[1..].parse::<i32>().unwrap();
    Turn {
        direction: dir,
        amount,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let password = part1(input);
        assert_eq!(password, 3);
    }

    #[test]
    fn test_turn() {
        assert_eq!(
            apply_turn(
                50,
                Turn {
                    direction: Direction::Left,
                    amount: 68
                }
            ),
            82
        );

        assert_eq!(
            apply_turn(
                52,
                Turn {
                    direction: Direction::Right,
                    amount: 48
                }
            ),
            0
        );
    }
}
