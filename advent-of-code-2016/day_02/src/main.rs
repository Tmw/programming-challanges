/*
--- Day 2: Bathroom Security ---

You arrive at Easter Bunny Headquarters under cover of darkness. However, you left in such a rush that you forgot to use the bathroom! Fancy office buildings like this one usually have keypad locks on their bathrooms, so you search the front desk for the code.

"In order to improve security," the document you find says, "bathroom codes will no longer be written down. Instead, please memorize and follow the procedure below to access the bathrooms."

The document goes on to explain that each button to be pressed can be found by starting on the previous button and moving to adjacent buttons on the keypad: U moves up, D moves down, L moves left, and R moves right. Each line of instructions corresponds to one button, starting at the previous button (or, for the first line, the "5" button); press whatever button you're on at the end of each line. If a move doesn't lead to a button, ignore it.

You can't hold it much longer, so you decide to figure out the code as you walk to the bathroom. You picture a keypad like this:

1 2 3
4 5 6
7 8 9
Suppose your instructions are:

ULL
RRDDD
LURDL
UUUUD
You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"), so the first button is 1.
Starting from the previous button ("1"), you move right twice (to "3") and then down three times (stopping at "9" after two moves and ignoring the third), ending up with 9.
Continuing from "9", you move left, up, right, down, and left, ending with 8.
Finally, you move up four times (stopping at "2"), then down once, ending with 5.
So, in this example, the bathroom code is 1985.

Your puzzle input is the instructions from the document you found at the front desk. What is the bathroom code?

--- Part Two ---

You finally arrive at the bathroom (it's a several minute walk from the lobby so visitors can behold the many fancy conference rooms and water coolers on this floor) and go to punch in the code. Much to your bladder's dismay, the keypad is not at all like you imagined it. Instead, you are confronted with the result of hundreds of man-hours of bathroom-keypad-design meetings:

    1
  2 3 4
5 6 7 8 9
  A B C
    D
You still start at "5" and stop when you're at an edge, but given the same instructions as above, the outcome is very different:

You start at "5" and don't move at all (up and left are both edges), ending at 5.
Continuing from "5", you move right twice and down three times (through "6", "7", "B", "D", "D"), ending at D.
Then, from "D", you move five more times (through "D", "B", "C", "C", "B"), ending at B.
Finally, after five more moves, you end at 3.
So, given the actual keypad layout, the code would be 5DB3.

Using the same instructions in your puzzle input, what is the correct bathroom code?

*/
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