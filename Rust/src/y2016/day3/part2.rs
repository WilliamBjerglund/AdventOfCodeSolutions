//!--- Day 3: Squares With Three Sides ---
/*
The idea here is in my mind simple.
in Part 1, each row was one triangle
in Part 2, 3 rows contains three triangles vertically so essentially a transposition.

The easy way to solve this is to almost reuse part 1.

First we read and parse all lines, into rows of numbers then we gorup those rows in chunks of three.
That means each chunk contains 3 triangles where each column will be the sides.
so now we just loop through collect values check them like before and move on.
*/

pub fn main(input: &str) -> i32 {
    let mut valid_triangles = 0;

    // Parse each line into a row of triangle side lengths.
    let rows: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Process the rows in groups of three to check for valid triangles.
    for row_group in rows.chunks_exact(3) {
        for column_index in 0..3 {
            let a = row_group[0][column_index];
            let b = row_group[1][column_index];
            let c = row_group[2][column_index];

            if a + b > c && a + c > b && b + c > a {
                valid_triangles += 1;
            }
        }
    }

    valid_triangles
}

#[test]
fn test() {
    let input = "\
101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";

    assert_eq!(main(input), 6);
}
