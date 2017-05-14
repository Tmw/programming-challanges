use std::io::prelude::*;
use std::fs::File;

mod command;
mod rotation;
mod dimension;
mod target;
mod display;
mod grid;
mod utils;

use command::Command;
use display::Display;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    let mut display = Display::new(50, 6); //w: 50, h: 6
    let commands: Vec<Command> = input.lines().map(|s| s.parse().unwrap()).collect();

    for c in commands {
        display.execute(c);
    }

    // answer to part A
    println!("Number of pixels lit is: {:?}", display.pixels_lit());
}
