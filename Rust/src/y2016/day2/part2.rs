// ! --- Day 2: Bathroom Security ---
// ! Part 2: The Bathroom Code

/*
The problem is almost the same here we can change out the rgrid for a new one with empty squares.
we then update our starting position which changed and finally instead of moving using rules we now instead define -
a potential move.
*/

#[rustfmt::skip]
pub fn main(input: &str) -> String {
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    let mut code = String::new();

    let mut x: usize = 0; // Start at 5 which is at (0, 2)
    let mut y: usize = 2;

    for line in input.lines() {
        for c in line.chars() {
            let (nx, ny) = match c {
                'U' => (x, y.saturating_sub(1)),
                'D' => (x, (y + 1).min(4)),
                'L' => (x.saturating_sub(1), y),
                'R' => ((x + 1).min(4), y),
                _ => (x, y),
            };

            // Only move if the destination is a real entry
            if keypad[ny][nx] != ' ' {
                x = nx;
                y = ny;
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
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let mut code = String::new();

    // Start at 5
    let mut x: usize = 0;
    let mut y: usize = 2;

    for line in input.lines() {
        for c in line.chars() {
            let (nx, ny) = match c {
                'U' => (x, y.saturating_sub(1)),
                'D' => (x, (y + 1).min(4)),
                'L' => (x.saturating_sub(1), y),
                'R' => ((x + 1).min(4), y),
                _ => (x, y),
            };

            if keypad[ny][nx] != ' ' {
                x = nx;
                y = ny;
            }
        }

        code.push(keypad[y][x]);
    }

    assert_eq!(code, "5DB3");
}
