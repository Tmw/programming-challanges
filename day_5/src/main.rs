/*
--- Day 5: How About a Nice Game of Chess? ---

You are faced with a security door designed by Easter Bunny engineers that seem to have acquired most of their security knowledge by watching hacking movies.

The eight-character password for the door is generated one character at a time by finding the MD5 hash of some Door ID (your puzzle input) and an increasing integer index (starting with 0).

A hash indicates the next character in the password if its hexadecimal representation starts with five zeroes. If it does, the sixth character in the hash is the next character of the password.

For example, if the Door ID is abc:

The first index which produces a hash that starts with five zeroes is 3231929, which we find by hashing abc3231929; the sixth character of the hash, and thus the first character of the password, is 1.
5017308 produces the next interesting hash, which starts with 000008f82..., so the second character of the password is 8.
The third time a hash starts with five zeroes is for abc5278568, discovering the character f.
In this example, after continuing this search a total of eight times, the password is 18f47a30.

Given the actual Door ID, what is the password?
*/

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    // make an empty string to store our password in
    let mut pass = String::new();

    // iterate forever until we break
    for i in 0..std::u64::MAX {
        // make a new MD5 hasher
        let mut hash = Md5::new();

        // concat our puzzle input with the index
        let input : &str = &format!("reyedfim{}", i);

        // the the concatenated string as input for our hasher
        hash.input_str(&input);

        // grab the result from the hash
        let result = hash.result_str();

        // grab the first 5 characters of the calculated hash
        let res_check = &result[..5];

        // if it starts with five zeroes, we have a match
        if res_check == "00000" {

            // what is the first character after those five zeroes?
            let next_char : char = result.chars().nth(5).unwrap();
            println!("found {:?}", next_char);

            // well thats part of our password. Lets store it 😜
            pass.push(next_char);

            // we don't quit until our calculated password is 8 character long
            if pass.len() == 8 {
                break;
            }

        }
    }

    // we got em. 😏
    println!("cracked the pass: {}", pass);
}
