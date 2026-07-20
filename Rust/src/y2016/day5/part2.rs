// ! --- Day 5: How About a Nice Game of Chess? ---

/*
For part 2 we now add the requirement that the sixth character of the hash is the position in the password and the seventh character is the value to put in that position.
To do that we refactored part1 so we can reuse the loop and hex conversion.
That way now we can just use sixth digit as position ignore all already filled positions and go until all 8 positions are filled.
*/

use super::part1::{next_interesting_hash, to_hex_character};

pub fn main(input: &str) -> String {
    let mut password: [Option<char>; 8] = [None; 8];
    let mut counter = 0;
    let mut hash_input = String::with_capacity(input.len() + 10);
    let mut filled_positions = 0;

    while filled_positions < 8 {
        let (sixth, seventh) = next_interesting_hash(input, &mut counter, &mut hash_input);

        // if the position is between 0 and 7 and not already filled we fill it and increase the filled positions counter.
        if sixth < 8 && password[sixth as usize].is_none() {
            password[sixth as usize] = Some(to_hex_character(seventh));
            filled_positions += 1;
        }
    }
    password.iter().map(|c| c.unwrap()).collect()
}
