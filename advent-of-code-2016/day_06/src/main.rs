/*
--- Day 6: Signals and Noise ---

Something is jamming your communications with Santa. Fortunately, your signal is only partially jammed, and protocol in situations like this is to switch to a simple repetition code to get the message through.

In this model, the same message is sent repeatedly. You've recorded the repeating message signal (your puzzle input), but the data seems quite corrupted - almost too badly to recover. Almost.

All you need to do is figure out which character is most frequent for each position. For example, suppose you had recorded the following messages:

eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
The most common character in the first column is e; in the second, a; in the third, s, and so on. Combining these characters returns the error-corrected message, easter.

Given the recording in your puzzle input, what is the error-corrected version of the message being sent?

Your puzzle answer was gebzfnbt.

--- Part Two ---

Of course, that would be the message - if you hadn't agreed to use a modified repetition code instead.

In this modified code, the sender instead transmits what looks like random data, but for each character, the character they actually want to send is slightly less likely than the others. Even after signal-jamming noise, you can look at the letter distributions in each column and choose the least common letter to reconstruct the original message.

In the above example, the least common character in the first column is a; in the second, d, and so on. Repeating this process for the remaining characters produces the original message, advent.

Given the recording in your puzzle input and this new decoding methodology, what is the original message that Santa is trying to send?
*/

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

type Column   = HashMap<char, usize>;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    let line_length = input.chars().count() / input.lines().count();
    let mut columns : Vec<Column> = Vec::with_capacity(line_length);

    for _ in 0..line_length {
        columns.push(Column::new())
    }

    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            let val = match columns[idx].get(&c) {
                Some(val) => val + 1,
                None => 1,
            };

            columns[idx].insert(c, val);
        }
    }

    let message_a : String = columns.iter()
        .map(|c| *sort_column(&c).last().unwrap())
        .collect();
    println!("[PART A] message: {}", message_a);

    let message_b : String = columns.iter()
        .map(|c| *sort_column(&c).first().unwrap())
        .collect();
    println!("[PART B] message: {}", message_b);
}

fn sort_column(column : &Column) -> Vec<char> {
    let mut thingy : Vec<(&char, &usize)> = column.iter().collect();
    thingy.sort_by_key(|r| r.1);
    thingy.iter().map(|r| *r.0).collect()
}