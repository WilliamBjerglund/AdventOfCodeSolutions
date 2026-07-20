// ! --- Day 9: Explosives in Cyberspace ---
/*
Here we can just reuse part 1 almost to the tea.

However instead of immediatly adding A x B we instead calculate the decompressed length of all characters net to A because they could contain markers.
so we add a bit of shitty cursion here.
*/

pub fn main(input: &str) -> usize {
    // Remove whitespace
    let input: String = input
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();

    // Recursively calculate and return the decompressed length.
    decompressed_length(&input)
}

fn decompressed_length(input: &str) -> usize {
    let mut total_length = 0;
    let mut current_pos = 0;

    while current_pos < input.len() {
        // if the current char is not '(': its a normal char
        if input.as_bytes()[current_pos] != b'(' {
            total_length += 1;
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

            // find where the repeated section starts one position after  the closing parenthesis
            let repeated_section_start = closing_paren_pos + 1;

            // find where the repeated section ends
            let repeated_section_end = repeated_section_start + amount_of_characters;

            // now we make a slice containing that section
            let repeated_section = &input[repeated_section_start..repeated_section_end];

            // Finally now we can recursively calculate the fully decompressed length of that section.
            let section_length = decompressed_length(repeated_section);

            // add to decompressed length the amount of characters times the repetitions
            total_length += section_length * repetitions;
            // finally move current pos to the end of the compressed section
            current_pos = repeated_section_end;
        }
    }
    total_length
}

#[test]
fn test() {
    assert_eq!(main("(3x3)XYZ"), 9);

    assert_eq!(main("X(8x2)(3x3)ABCY"), 20);

    assert_eq!(main("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241_920);

    assert_eq!(
        main("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
        445
    );
}
