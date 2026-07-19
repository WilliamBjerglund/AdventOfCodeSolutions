// !--- Day 3: Squares With Three Sides ---
/*
Very simple day we get a file like this
123 123 123
123 123 123
123 123 123

Here the goal is to just take each line take number a and b then add them together and if they are greater than C then add 1 to the valid triangle counter
if they are not then just continue to the next line and do the same thing until we reach the end of the file
*/

pub fn main(input: &str) -> i32 {
    let mut valid_triangles = 0;

    for line in input.lines() {
        let sides: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if sides.len() == 3 {
            let a = sides[0];
            let b = sides[1];
            let c = sides[2];

            if a + b > c && a + c > b && b + c > a {
                valid_triangles += 1;
            }
        }
    }

    valid_triangles
}

#[test]
fn test() {
    let input = "5 10 25\n3 4 5\n6 8 10\n1 2 3";
    assert_eq!(main(input), 2);
}
