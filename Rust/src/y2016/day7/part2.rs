// ! --- Day 7: Internet Protocol Version 7 ---
/*
Now we want to do almost the same this time we would also like to know which IPS support SSL
This is just ABA outside of brackets and BAB inside of brackets.
*/

pub fn main(input: &str) -> usize {
    let mut valid_count = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut inside_brackets = false;
        let mut outside_patterns: Vec<(u8, u8)> = Vec::new(); // Store ABA patterns outside brackets
        let mut inside_patterns: Vec<(u8, u8)> = Vec::new(); // Store BAB patterns inside brackets
        for i in 0..bytes.len() {
            if bytes[i] == b'[' {
                inside_brackets = true;
                continue;
            }
            if bytes[i] == b']' {
                inside_brackets = false;
                continue;
            }

            // this time we need 3 characters to check for ABA/BAB patterns
            if i + 2 >= bytes.len() {
                break;
            }

            let first = bytes[i];
            let second = bytes[i + 1];
            let third = bytes[i + 2];

            // Now we can ignore groups of 3 that cross the brackets.
            if first == b'['
                || first == b']'
                || second == b'['
                || second == b']'
                || third == b'['
                || third == b']'
            {
                continue;
            }

            // now we check whether our 3 characters form an ABA pattern
            let is_aba = first == third && first != second;

            if is_aba {
                if inside_brackets {
                    inside_patterns.push((first, second)); // Store as BAB pattern
                } else {
                    outside_patterns.push((first, second)); // Store as ABA pattern
                }
            }
        }

        // if the outside is a, b then the inside must be b, a
        for (first, second) in outside_patterns {
            if inside_patterns.contains(&(second, first)) {
                valid_count += 1;
                break;
            }
        }
    }
    valid_count
}

#[test]
fn test() {
    let input = "\
aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";

    assert_eq!(main(input), 3);
}
