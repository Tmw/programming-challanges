/*
--- Day 4: Security Through Obscurity ---

Finally, you come across an information kiosk with a list of rooms. Of course, the list is encrypted and full of decoy data, but the instructions to decode the list are barely hidden nearby. Better remove the decoy data first.

Each room consists of an encrypted name (lowercase letters separated by dashes) followed by a dash, a sector ID, and a checksum in square brackets.

A room is real (not a decoy) if the checksum is the five most common letters in the encrypted name, in order, with ties broken by alphabetization. For example:

aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are a (5), b (3), and then a tie between x, y, and z, which are listed alphabetically.
a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each), the first five are listed alphabetically.
not-a-real-room-404[oarel] is a real room.
totally-real-room-200[decoy] is not.
Of the real rooms from the list above, the sum of their sector IDs is 1514.

What is the sum of the sector IDs of the real rooms?

--- Part Two ---

With all the decoy data out of the way, it's time to decrypt this list and get moving.

The room names are encrypted by a state-of-the-art shift cipher, which is nearly unbreakable without the right software. However, the information kiosk designers at Easter Bunny HQ were not expecting to deal with a master cryptographer like yourself.

To decrypt a room name, rotate each letter forward through the alphabet a number of times equal to the room's sector ID. A becomes B, B becomes C, Z becomes A, and so on. Dashes become spaces.

For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted name.

What is the sector ID of the room where North Pole objects are stored?

*/

extern crate regex;
#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Room {
    name : String,
    sector_id : isize,
    checksum: String,
}

impl Room {
    fn from_line(line : &str) -> Room {

        lazy_static! {
            // compile regex only once
            static ref REGEX : Regex = Regex::new(r"^((([a-zA-Z0-9]+\-)+)([0-9]+)\[(.+)\])$").unwrap();        
        }

        // find matches in line
        let cap = REGEX.captures(line).unwrap();

        // grab the parts we need
        let name : String     = cap.get(2).unwrap().as_str().to_string();
        let sector_id : isize = cap.get(4).unwrap().as_str().parse().unwrap();
        let checksum : String = cap.get(5).unwrap().as_str().to_string();

        // and create a Room struct off of it
        Room { name: name, sector_id: sector_id, checksum: checksum }
    }

    // does checksum match input?
    fn is_valid(&self) -> bool {
        // remove the dashes
        let undashed_name : String = self.name.replace('-', "");

        // grab the chars as an array
        let mut chars_as_vec : Vec<char> = undashed_name.chars().collect();

        // sort the array alphabetically
        chars_as_vec.sort();

        // dedupe the array
        chars_as_vec.dedup();

        // create tuples with the character and numer of occurances in original string
        let mut char_and_count : Vec<(char, usize)> = chars_as_vec.iter()
            .map(|c| { (*c, undashed_name.matches(*c).count()) })
            .collect();
        
        // ordering.. magic :)
        char_and_count.sort_by(|a, b| {
            // if a.1 (number of occurances) equal..
            if a.1 == b.1 {
                // matching; compare using the char alphabetically
                return b.0.cmp(&a.0)
            }
            // not matching; how do they compare?
            a.1.cmp(&b.1)
        });

        // since the order is backwards, lets reverse
        char_and_count.reverse();

        // truncate to first five chars
        char_and_count.truncate(5);

        // grab the chars and concatenate as string
        let checksum_as_vec : Vec<char> = char_and_count.iter().map(|c| c.0).collect();
        let checksum : String = checksum_as_vec.clone().into_iter().collect();

        // a room is valid once the two checksums match
        return checksum == self.checksum
    }

    fn decipher(&self) -> String {
        let undashed_name = self.name.replace('-', " ");
        rot(undashed_name, self.sector_id as usize)
    }
}

// basic ceasar cipher implementation
fn rot(input : String, rotations : usize) -> String {
    let alphabet  = "abcdefghijklmnopqrstuvwxyz".to_string();

    input.chars()
        .map(|c| {
            if c == ' ' { return c; }
            let new_index = alphabet.find(c).unwrap() + rotations;
            alphabet.chars().cycle().nth(new_index).unwrap()
        })
        .collect()

        //  ^ have you seen this cycle thing? 😯
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    let rooms : Vec<Room> = input.lines().map(|line| {Room::from_line(line)}).collect();

    // grab the valid rooms
    let valid_rooms : Vec<&Room> = rooms.iter()
        .filter(|r| {r.is_valid()})
        .collect();
    
    // and sum the sector id using fold
    let sum = valid_rooms.iter().fold(0, |sum, r| { sum + r.sector_id});

    // for part B we're looking for a room where north pole objects are stored
    let deciphered_names : Vec<(isize, String)> = valid_rooms.iter()
        .map(|r| (r.sector_id, r.decipher() ) )
        .collect();

    // print what can be found where 💪
    for (sector_id, name) in deciphered_names {
        println!("{}can be found in sector{}", name, sector_id);
    }

    // print the answer 🎉
    println!("sum: {}", sum);
}

#[test]
fn test_rot() {
    let res = rot("mec".to_string(), 10);
    assert_eq!(res, "wom".to_string())
}

#[test]
fn test_rot_for_realsies(){
    let res = rot("qzmt zixmtkozy ivhz".to_string(), 343);
    assert_eq!(res, "very encrypted name".to_string());
}
