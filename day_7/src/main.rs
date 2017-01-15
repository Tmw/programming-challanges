// --- Day 7: Internet Protocol Version 7 ---
//
// While snooping around the local network of EBHQ, you compile a list of IP addresses (they're IPv7, of course; IPv6 is much too limited). You'd like to figure out which IPs support TLS (transport-layer snooping).
//
// An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An ABBA is any four-character sequence which consists of a pair of two different characters followed by the reverse of that pair, such as xyyx or abba. However, the IP also must not have an ABBA within any hypernet sequences, which are contained by square brackets.
//
// For example:
//
// abba[mnop]qrst supports TLS (abba outside square brackets).
// abcd[bddb]xyyx does not support TLS (bddb is within square brackets, even though xyyx is outside square brackets).
// aaaa[qwer]tyui does not support TLS (aaaa is invalid; the interior characters must be different).
// ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets, even though it's within a larger string).
// How many IPs in your puzzle input support TLS?
//
// --- Part Two ---
//
// You would also like to know which IPs support SSL (super-secret listening).
//
// An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere in the supernet sequences (outside any square bracketed sections), and a corresponding Byte Allocation Block, or BAB, anywhere in the hypernet sequences. An ABA is any three-character sequence which consists of the same character twice with a different character between them, such as xyx or aba. A corresponding BAB is the same characters but in reversed positions: yxy and bab, respectively.
//
// For example:
//
// aba[bab]xyz supports SSL (aba outside square brackets with corresponding bab within square brackets).
// xyx[xyx]xyx does not support SSL (xyx, but no corresponding yxy).
// aaa[kek]eke supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related, because the interior character must be different).
// zazbz[bzb]cdb supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz and zbz overlap).
//
// How many IPs in your puzzle input support SSL?
//

extern crate regex;
#[macro_use]
extern crate lazy_static;

mod ip_address;
mod part_a_validator;
mod part_b_validator;

use std::io::prelude::*;
use std::fs::File;
use ip_address::IPAddress;

use part_a_validator::PartAValidator;
use part_b_validator::PartBValidator;


fn main() {
    // read input from file
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    // parse lines to IPAddress objects
    let addresses: Vec<IPAddress> = input.lines().map(|l| l.parse().unwrap()).collect();

    // figure out answer A
    let part_a_validator = PartAValidator{};
    let answer_a: Vec<&IPAddress> = addresses.iter().filter(|a| a.is_valid(&part_a_validator)).collect();
    println!("[PART A] answer: {}", answer_a.len());

    // figure out answer B
    let part_b_validator = PartBValidator{};
    let answer_b: Vec<&IPAddress> = addresses.iter().filter(|a| a.is_valid(&part_b_validator)).collect();
    println!("[PART B] answer: {}", answer_b.len());
}
