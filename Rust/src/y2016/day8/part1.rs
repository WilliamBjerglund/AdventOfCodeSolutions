// ! --- Day 8: Two-Factor Authentication ---
/*
This one is a bit weird we have a screen that is 50x6 but it is smashed so we have to simulate what is displayed.

The idea for me here is to make a 2d grid of bools false is off true is on.
Then read each instruction and perform the operation on the grid.
Finally i will loop through and count how many lit pixels there are

*/
pub fn build_screen(input: &str) -> [[bool; 50]; 6] {
    // make the screen
    let mut screen = [[false; 50]; 6];

    for line in input.lines() {
        if line.starts_with("rect ") {
            let dimensions = line.strip_prefix("rect ").unwrap();
            let (width, height) = dimensions.split_once('x').unwrap();

            let width = width.parse::<usize>().unwrap();
            let height = height.parse::<usize>().unwrap();

            // perform rectangle operation so rect 3x2 means width 3 and height 2 turn on from top left corner
            for row in 0..height {
                for col in 0..width {
                    screen[row][col] = true;
                }
            }
        } else if line.starts_with("rotate row") {
            // read row number after y= in something like rotate row y=0 by 4
            let row_number = line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .strip_prefix("y=")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let shift_amount = line
                .split_whitespace()
                .nth(4)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            // temp row containing only false
            let mut temp_row = [false; 50];

            for col in 0..50 {
                // calculate new column
                let new_col = (col + shift_amount) % 50;
                // copy current pixel into temp row at new column
                temp_row[new_col] = screen[row_number][col];
            }

            // Replace the original row with the temp row
            screen[row_number] = temp_row;
        } else if line.starts_with("rotate column") {
            // read column number after x= in something like rotate column x=1 by 1
            let col_number = line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .strip_prefix("x=")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let shift_amount = line
                .split_whitespace()
                .nth(4)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            // again temp column containing only false
            let mut temp_col = [false; 6];

            for row in 0..6 {
                // calculate new row
                let new_row = (row + shift_amount) % 6;
                // copy current pixel into temp column at new row
                temp_col[new_row] = screen[row][col_number];
            }

            // Replace the original column with the temp column
            for row in 0..6 {
                screen[row][col_number] = temp_col[row];
            }
        }
    }

    screen
}

pub fn main(input: &str) -> u32 {
    let screen = build_screen(input);
    let mut lit_pixels = 0;

    for row in screen {
        for pixel in row {
            if pixel {
                lit_pixels += 1;
            }
        }
    }

    lit_pixels
}
#[test]
fn test() {
    let input = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    assert_eq!(main(input), 6);
}
