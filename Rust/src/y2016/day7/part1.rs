// ! --- Day 7: Internet Protocol Version 7 ---
/*
Today is a weird one, we have to find out how many IPs support TLS following the IPv7 format which is dumb.
the idea here is it supports TLS if it has an ABBA pattern outside of brackets and no ABBA pattern inside brackets.
So we can just split the input into two parts, one with the brackets and one without.
Then we just check if the part without brackets has an ABBA pattern and the part with brackets does not have an ABBA pattern.
*/

pub fn main(input: &str) -> usize {
    let mut valid_count = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut inside_brackets = false;
        let mut has_abba_outside = false;
        let mut has_abba_inside = false;

        for i in 0..bytes.len() {
            // Check for brackets to determine if we are inside or outside brackets
            if bytes[i] == b'[' {
                inside_brackets = true;
                continue;
            } else if bytes[i] == b']' {
                inside_brackets = false;
                continue;
            }
            // Stop when fewer than four characters remain.
            if i + 3 >= bytes.len() {
                break;
            }

            // Check for ABBA pattern
            let first = bytes[i];
            let second = bytes[i + 1];
            let third = bytes[i + 2];
            let fourth = bytes[i + 3];

            let is_abba = first == fourth && second == third && first != second;

            if is_abba {
                if inside_brackets {
                    has_abba_inside = true;
                } else {
                    has_abba_outside = true;
                }
            }
        }

        if has_abba_outside && !has_abba_inside {
            valid_count += 1;
        }
    }
    valid_count
}

#[test]
fn test() {
    let input = "\
        abba[mnop]qrst
        abcd[bddb]xyyx
        aaaa[qwer]tyui
        ioxxoj[asdfgh]zxcvbn";

    assert_eq!(main(input), 2);
}
