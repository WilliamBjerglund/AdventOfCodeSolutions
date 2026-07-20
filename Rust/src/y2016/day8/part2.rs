// ! --- Day 8: Two-Factor Authentication ---
/*
This time we want to see what code the screen is trying to display.
to that we can just build the screen using the code from part 1 then for each row in the screen create an output row meaning
we simply go trough the pixels and if its on append a # if off just ' ' empty string.
*/

use super::part1::build_screen;

pub fn main(input: &str) -> String {
    let screen = build_screen(input);
    let mut output = String::new();

    for row in screen.iter() {
        for &pixel in row.iter() {
            if pixel {
                output.push('█');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    output
}

// Figuring out this test is a bit fucked their example is 7 x 3 so its not the same.....
#[test]
fn test() {
    let input = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    let expected_rows = ["    █ █", "█ █", " █", " █", "", ""];

    let expected: String = expected_rows
        .iter()
        .map(|row| format!("{row:<50}\n"))
        .collect();

    assert_eq!(main(input), expected);
}
