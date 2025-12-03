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

fn part1(input: &str) -> i32 {
    let turns: Vec<_> = input.lines().map(parse_turn).collect();

    let mut password: i32 = 0;
    let mut dial = 50;
    for turn in turns {
        let (new_dial, zeroes) = apply_turn(dial, turn);
        dial = new_dial;
        password += zeroes;
    }

    password
}

fn apply_turn(n: i32, turn: Turn) -> (i32, i32) {
    let mut dial = n;
    let mut zeroes = 0;
    let mut amount = turn.amount;
    while amount > 0 {
        let result = match turn.direction {
            Direction::Left => dial - 1,
            Direction::Right => dial + 1,
        };

        dial = result % 100;
        if dial == 0 {
            zeroes += 1;
        }
        amount -= 1;
    }

    (dial, zeroes)
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
        assert_eq!(password, 6);
    }

    // #[test]
    // fn test_turn() {
    //     assert_eq!(
    //         apply_turn(
    //             50,
    //             Turn {
    //                 direction: Direction::Left,
    //                 amount: 68
    //             }
    //         ),
    //         82
    //     );
    //
    //     assert_eq!(
    //         apply_turn(
    //             52,
    //             Turn {
    //                 direction: Direction::Right,
    //                 amount: 48
    //             }
    //         ),
    //         0
    //     );
    // }
}
