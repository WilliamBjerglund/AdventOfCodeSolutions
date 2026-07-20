// ! --- Day 9: Explosives in Cyberspace ---
/*
This was a ducey,
We are told we find a file that is compressed using a simple format that is new. we have the documentation for it.
the format compresses a sequence of characters where whitespace is ignoreed to indicidate that some sequence should be repeated.
a marker gets added to the file say 10x2 where we take the next 10 characters and repeat them 2 times. then continue.
if we reach parentheses or other characters appear by a marker that is okay just treat it as normal data.
finally we are asked to find the decompressed length of the file.

My idea here is rather dumb, we don't actually care decompressing it cause we just want the length.
So we can move through the input one character at a time and just add one to the length each normal character.
When we find markers we just add A * B to the total then skip both the marker and the characters it describes.
*/

pub fn main(input: &str) -> usize {
    let input = input.trim();
    let mut decompressed_length = 0;
    let mut current_pos = 0;

    while current_pos < input.len() {
        // if the current char is not '(': its a normal char
        if input.as_bytes()[current_pos] != b'(' {
            decompressed_length += 1;
            current_pos += 1;
        } else {
            // find end
            let closing_paren_pos = input[current_pos..].find(')').unwrap() + current_pos;
            // read marker say 3x4 etc.... and split
            let marker = &input[current_pos + 1..closing_paren_pos];
            let mut marker_parts = marker.split('x');
            // first number gives us the amount of chars to use
            let amount_of_characters: usize = marker_parts.next().unwrap().parse().unwrap();
            // tells us how many times repeated
            let repetitions: usize = marker_parts.next().unwrap().parse().unwrap();

            // add to decompressed length the amount of characters times the repetitions
            decompressed_length += amount_of_characters * repetitions;
            // skip past the end and chars already covered.
            current_pos = closing_paren_pos + 1 + amount_of_characters;
        }
    }
    decompressed_length
}

#[test]
fn test() {
    assert_eq!(main("ADVENT"), 6);
    assert_eq!(main("A(1x5)BC"), 7);
    assert_eq!(main("(3x3)XYZ"), 9);
    assert_eq!(main("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(main("(6x1)(1x3)A"), 6);
    assert_eq!(main("X(8x2)(3x3)ABCY"), 18);
}
