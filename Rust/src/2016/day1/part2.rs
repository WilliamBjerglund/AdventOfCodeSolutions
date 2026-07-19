//! --- Day 1: No Time for a Taxicab ---
//! Part 2

use crate::part1::{Direction, Position};
use std::collections::HashSet;

pub fn main(input: &str) -> i32 {
    let mut position = Position {
        x: 0,
        y: 0,
        direction: Direction::North,
    };

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((position.x, position.y));

    for instructions in input.trim().split(",") {
        let (turn, distance) = instructions.trim().split_at(1);
        let distance: i32 = distance.parse().unwrap();

        position.direction = match turn {
            "L" => position.direction.turn_left(),
            "R" => position.direction.turn_right(),
            _ => panic!("Invalid turn"),
        };

        for _ in 0..distance {
            match position.direction {
                Direction::North => position.y += 1,
                Direction::East => position.x += 1,
                Direction::South => position.y -= 1,
                Direction::West => position.x -= 1,
            }

            if visited.contains(&(position.x, position.y)) {
                return position.x.abs() + position.y.abs();
            } else {
                visited.insert((position.x, position.y));
            }
        }
    }

    panic!("No location visited twice");
}

#[test]
fn part2_test() {
    assert_eq!(main("R8, R4, R4, R8"), 4);
}
