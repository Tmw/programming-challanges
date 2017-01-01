use std::io::prelude::*;
use std::fs::File;
use std::ops::Rem;

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

type Cursor = usize;
type Numpad = Vec<char>;

fn main() {
    // read input from the file
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    // split into lines
    let lines : Vec<&str> = input.split('\n').collect();

    let mut digits : Vec<char> = Vec::new();

    // recreate the keypad with three rows of three keys
    let keypad : Numpad = vec!['1','2','3','4','5','6','7','8','9'];

    // and lets create our cursor, starting at the Five key
    let mut cursor : Cursor = 4;

    // convert characters to directions
    for line in lines {

        let sequence : Vec<Direction> = line.chars().map(char_to_direction).collect();

        let updated_cursor_after_sequence : Cursor = sequence.iter().fold(cursor, |prev_cursor, dir| { 
            move_cursor(&keypad, &prev_cursor, dir)
        });

        digits.push(digit_for_cursor(&keypad, &updated_cursor_after_sequence));
        cursor = updated_cursor_after_sequence;
    }

    println!("Digits: {:?}", digits);
    assert_eq!(digits, vec!['9', '9', '3', '3', '2']); //challange (live) digits
}

fn char_to_direction(c : char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
         _  => Direction::Up,
    }
}

fn move_cursor(numpad : &Numpad, cursor: &Cursor, dir : &Direction) -> Cursor {
    // calculate new position of the cursor, based on the direction
    // and have it be constraint to the bounds of the numpad
    let row_length = f32::sqrt(numpad.len() as f32) as usize;
    
    match *dir {
        Direction::Left => {
            // left most options? Cant move left any further..
            if cursor.rem(row_length) == 0 { *cursor } else { cursor -1 }
        },

        Direction::Right => {
            // right most options? Cant move right any further..
            if (cursor).rem(row_length) == row_length - 1 { *cursor } else { cursor +1 }
        },

        Direction::Up => {
            // only can move up if theres another row up top
            if *cursor > row_length - 1 { cursor - row_length } else { *cursor }
        },

        Direction::Down => {
            // only can move down if theres another row below
            if *cursor < numpad.len() - row_length { cursor + row_length } else { *cursor }
        },
    }
}

fn digit_for_cursor(numpad : &Numpad, cursor : &Cursor) -> char {
   numpad.get(*cursor).unwrap().to_owned()
}