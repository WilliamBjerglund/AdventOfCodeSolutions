// ! --- Day 6: Signals and Noise ---
/*
Today is rather easy, the idea i have here is to just read the length of the first row now we know how many columns we have.
Then we just create a frequency counter for each column and go row by row using the length from earlier to append to each columns own list

finally we make a empty message and for each column we find the most common character and append it to the message and return it.
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
        let mut most_common = ' ';
        let mut highest_count = 0;

        for (char, count) in column {
            if count > highest_count {
                highest_count = count;
                most_common = char;
            }
        }
        message.push(most_common);
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

    assert_eq!(main(input), "easter");
}
