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
type Numpad = Vec<Option<char>>;

fn main() {
    // read input from the file
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    // split into lines
    let lines : Vec<&str> = input.split('\n').collect();

    let mut digits : Vec<char> = Vec::new();

    // recreate the keypad with three rows of three keys
    // let keypad_part_a : Numpad = vec![Some('1'), Some('2'), Some('3'),
    //                            Some('4'), Some('5'), Some('6'),
    //                            Some('7'), Some('8'), Some('9')];

    let keypad : Numpad = vec![
        None,        None,       Some('1'),   None,       None,
        None,        Some('2'),  Some('3'),   Some('4'),  None,
        Some('5'),   Some('6'),  Some('7'),   Some('8'),  Some('9'),
        None,        Some('A'),  Some('B'),   Some('C'),  None,
        None,        None,       Some('D'),   None,       None
    ];

    // and lets create our cursor, starting at the Five key
    let mut cursor : Cursor = 10;

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
            if cursor.rem(row_length) == 0 || !is_valid(&numpad, &(cursor - 1)) { 
                *cursor 
            } else { 
                cursor - 1 
            }
        },

        Direction::Right => {
            // right most options? Cant move right any further..
            if (cursor).rem(row_length) == row_length - 1 || !is_valid(&numpad, &(cursor + 1)) { 
                *cursor 
            } else { 
                cursor + 1 
            }
        },

        Direction::Up => {
            // only can move up if theres another row up top
            if *cursor > row_length - 1 && is_valid(&numpad, &(cursor - row_length)) { 
                cursor - row_length 
            } else { 
                *cursor 
            }
        },

        Direction::Down => {
            // only can move down if theres another row below
            if *cursor < numpad.len() - row_length && is_valid(&numpad, &(cursor + row_length)){ 
                cursor + row_length 
            } else { 
                *cursor 
            }
        },
    }
}

fn is_valid(numpad: &Numpad, cursor : &Cursor) -> bool {
    numpad.get(*cursor).unwrap().is_some()
}

fn digit_for_cursor(numpad : &Numpad, cursor : &Cursor) -> char {
   numpad.get(*cursor).unwrap().unwrap().to_owned()
}