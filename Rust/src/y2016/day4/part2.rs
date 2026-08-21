// ! --- Day 4: Security Through Obscurity ---
/*
This one is simple enough a shift cipher.
Here we want to use part 1 to only get the real rooms then we want to shift each letter forward by the sector ID so if sector ID is 1.
then a -> B, B-> c and so on....
finally we convert a letter into a number so a = 0 and z = 25.
*/

use super::part1::main as part1_main;

fn decrypt_name(encrypted_name: &str, sector_id: u32) -> String {
    // Reduce the sector ID to a shift between 0 and 25
    let shift = (sector_id % 26) as u8;
    let mut decrypted_name = String::new();

    let base = b'a'; // ASCII value of 'a'

    // Here we loop over each character in the name if its a dash we convert to a space otherwise we shift the letter forward by the sector ID.
    for c in encrypted_name.chars() {
        if c == '-' {
            decrypted_name.push(' ');
        } else {
            let pos = (c as u8 - base + shift) % 26;
            decrypted_name.push((pos + base) as char);
        }
    }

    decrypted_name
}

pub fn main(input: &str) -> u32 {
    for line in input.lines() {
        // give a single room to part 1
        let valid_sector_id = part1_main(line);

        // if decoy skip it.
        if valid_sector_id == 0 {
            continue;
        }

        // split off checksum
        let (room_data, _checksum) = line.rsplit_once('[').unwrap();

        // split of name from id
        let (encrypted_name, _sector_id) = room_data.rsplit_once('-').unwrap();

        // decrypt the room name by shifting each letter forward by the sector ID.
        let decrypted_name = decrypt_name(encrypted_name, valid_sector_id);

        // Return the Id wen the target room is found
        if decrypted_name == "northpole object storage" {
            return valid_sector_id;
        }
    }

    panic!("Target room not found");
}

#[test]
fn test() {
    assert_eq!(
        decrypt_name("qzmt-zixmtkozy-ivhz", 343),
        "very encrypted name"
    );
}
