// ! --- Day 5: How About a Nice Game of Chess? ---
/*
Today is a very simple day, we are given a input "wtnhxymk" and asked to find the passwrod of the door that is eight characters long.

The password is a simple md5 where we take our input add a Door ID starting at 0 and increasing its integer until we find the first hash that -
gives 5 zeros in a row. Then the sixth character is our first input for password and then we continue until we have 8 charracters.

I will use the md5 crate for simplicity.
*/

use md5::{Digest, Md5};
use std::fmt::Write;

// ! Helper function made after finishing it because part 2 is basically the same so might as well just reuse via refactor.
pub fn next_interesting_hash(input: &str, counter: &mut u64, hash_input: &mut String) -> (u8, u8) {
    loop {
        hash_input.clear();
        hash_input.push_str(input);
        write!(hash_input, "{}", counter).unwrap();

        // Create a new Md5 hash and process the input string
        let mut hasher = Md5::new();
        hasher.update(hash_input.as_bytes());

        let result = hasher.finalize();

        *counter += 1;

        // two complete zero bytes and one where we take the upper part as 0.
        if result[0] == 0 && result[1] == 0 && result[2] < 0x10 {
            let sixth = result[2] & 0x0f;
            let seventh = result[3] >> 4;

            return (sixth, seventh);
        }
    }
}

pub fn to_hex_character(value: u8) -> char {
    b"0123456789abcdef"[value as usize] as char
}

pub fn main(input: &str) -> String {
    let mut password = String::with_capacity(8);
    let mut counter = 0;
    let mut hash_input = String::with_capacity(input.len() + 10);

    while password.len() < 8 {
        let (sixth, _seventh) = next_interesting_hash(input, &mut counter, &mut hash_input);

        password.push(to_hex_character(sixth));
    }
    password
}

#[test]
fn test_example() {
    let input = "abc";
    let expected = "18f47a30";

    assert_eq!(main(input), expected);
}
