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
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    let rooms : Vec<Room> = input.lines().map(|line| {Room::from_line(line)}).collect();

    // grab the valid rooms, and sum the sector id using fold
    let sum = rooms.iter()
        .filter(|r| {r.is_valid()})
        .fold(0, |sum, r| { sum + r.sector_id});

    // print the answer 🎉
    println!("sum: {}", sum);
}
