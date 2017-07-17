extern crate regex;
#[macro_use] extern crate lazy_static;

mod part_a;
mod part_b;

fn main() {
    let data = include_str!("input.txt");
    part_a::solve(&data);
    part_b::solve(&data);
}
