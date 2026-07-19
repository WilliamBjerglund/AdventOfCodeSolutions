// ! --- Day 2: Bathroom Security ---

/*
The instructions given in input.txt is a set of values D means Down U means Up L mean Left and R means Right.

We move fofllowing the instructions in the keypad starting with number 5. Each line is a set to get a number.
The key rule here is say you start with ULL well you go up to 2 then left to 1 but you cant go left again so you stay.

Once all instructions in a line are done you get a single digit that will be for line 1. the first digit of the code.
We then repeat this for all lines and get the code for the bathroom.

We can do this using a list as the storage to append the digits to and then a loop to go through each line.
*/

#[rustfmt::skip]
pub fn main(input: &str) -> String {
    let keypad = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ];

    let mut code = String::new();

    let mut x = 1; // Start at 5 which is at (1, 1)
    let mut y = 1;

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => if y > 0 { y -= 1; }, // Move up if not at the top row
                'D' => if y < 2 { y += 1; }, // Move down if not at the bottom row
                'L' => if x > 0 { x -= 1; }, // Move left if not at the leftmost column
                'R' => if x < 2 { x += 1; }, // Move right if not at the rightmost column
                _ => (),
            }
        }
        code.push(keypad[y][x]);
    }

    code
}

#[rustfmt::skip]
#[test]
fn test() {
    let keypad = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
    ];

    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let mut code = String::new();

    let mut x = 1; // Start at 5 which is at (1, 1)
    let mut y = 1;

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => if y > 0 { y -= 1; },
                'D' => if y < 2 { y += 1; },
                'L' => if x > 0 { x -= 1; },
                'R' => if x < 2 { x += 1; },
                _ => (),
            }
        }
        code.push(keypad[y][x]);
    }

    assert_eq!(code, "1985");
}
