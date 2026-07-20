// ! --- Day 4: Security Through Obscurity ---
/*
This one took me a while to figure out what it wants. but essentially its asking for a couple of things.
It asking us to split the line into name ID and checksum then count each letter in the name build a checksum that the room should have and compare it.
if they match then add it to the ID.

*/
use std::collections::HashMap;

pub fn main(input: &str) -> u32 {
    let mut sector_id_sum = 0;

    // First we parse each room and checksum split.
    for line in input.lines() {
        // split off checksum
        let (room_data, checksum) = line.rsplit_once('[').unwrap();
        let checksum = checksum.trim_end_matches(']');

        // split of name from id
        let (room_data, sector_id) = room_data.rsplit_once('-').unwrap();
        let sector_id = sector_id.parse::<u32>().unwrap();

        // now we count the letters without the dashes a hashmap is a good way to do this.
        let mut letter_count: HashMap<char, u32> = HashMap::new();

        for c in room_data.chars() {
            if c != '-' {
                // find the letter in the hashmap and increment the number if not there insert with count of 0.
                *letter_count.entry(c).or_insert(0) += 1;
            }
        }

        // Now we make our checksum to do that we start by turning the hashmap into a vector so we can sort it by frequency and then alphabetically if same frequency.
        let mut letters: Vec<(char, u32)> = letter_count.into_iter().collect();

        letters.sort_by(|a, b| {
            // first we sort by frequency
            let freq_order = b.1.cmp(&a.1);
            if freq_order == std::cmp::Ordering::Equal {
                // if the frequency is the same we sort by letter
                a.0.cmp(&b.0)
            } else {
                freq_order
            }
        });

        // now we build the checksum from the first 5 letters in the sorted vector.
        let expected_checksum: String = letters.iter().take(5).map(|(c, _count)| *c).collect();

        // finally validate by comparing checksum
        if expected_checksum == checksum {
            sector_id_sum += sector_id;
        }
    }
    sector_id_sum
}

#[test]
fn test() {
    let input = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    assert_eq!(main(input), 1514);
}
