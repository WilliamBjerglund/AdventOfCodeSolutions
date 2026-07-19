//! --- Day 1: No Time for a Taxicab ---
//! Part 1
/*
We only need to know where we are and which direction we are facing after an instruction.

So the idea i am going to make here is create a struct that holds and groups together the complete state of the traveler.
Then use a enum Direction for each valid direction N W E and S
I then make a turn left and right method that just says how we change direction when we turn left or right.

Then we just loop through the instructions and update the state of the traveler and at the end we return the distance from the original.
*/
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

pub fn main(input: &str) -> i32 {
    let mut position = Position {
        x: 0,
        y: 0,
        direction: Direction::North,
    };

    for instruction in input.trim().split(", ") {
        let (turn, distance) = instruction.split_at(1);
        let distance: i32 = distance.parse().unwrap();

        position.direction = match turn {
            "L" => position.direction.turn_left(),
            "R" => position.direction.turn_right(),
            _ => panic!("Invalid turn"),
        };

        match position.direction {
            Direction::North => position.y += distance,
            Direction::East => position.x += distance,
            Direction::South => position.y -= distance,
            Direction::West => position.x -= distance,
        }
    }

    position.x.abs() + position.y.abs()
}

#[test]
fn examples() {
    assert_eq!(main("R2, L3"), 5);
    assert_eq!(main("R2, R2, R2"), 2);
    assert_eq!(main("R5, L5, R5, R3"), 12);
}
