// ! --- Day 6: Signals and Noise ---
/*
Really simple.
Is it worth it? let me work it
i put my thing down, flip it and reverse it
ti esrever dna ti pilf, nwod gniht ym tup
ti esrever dna ti pilf, nwod gniht ym tup

If you know you know......
*/

use std::collections::HashMap;

pub fn main(input: &str) -> String {
    let columns_count = input.lines().next().unwrap().len();
    let mut columns: Vec<HashMap<char, u32>> = Vec::new(); // create a vector of hashmaps for each column

    for _ in 0..columns_count {
        columns.push(HashMap::new());
    }

    // Count each character in each column
    for line in input.lines() {
        for (idx, char) in line.chars().enumerate() {
            let column = &mut columns[idx];
            *column.entry(char).or_insert(0) += 1;
        }
    }

    let mut message = String::new();

    // find the most frequent character in each column and append to message
    for column in columns {
        let mut least_common = ' ';
        let mut lowest_count = u32::MAX;

        for (char, count) in column {
            if count < lowest_count {
                lowest_count = count;
                least_common = char;
            }
        }
        message.push(least_common);
    }
    message
}

#[test]
fn test() {
    let input = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    assert_eq!(main(input), "advent");
}
