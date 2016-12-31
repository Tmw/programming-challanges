enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq)]
struct Location {
    x : i32,
    y : i32,
}

impl Location {
    // return an initial location (x=0, y=0)
    pub fn initial() -> Location {
        return Location { x: 0, y: 0 };
    }

    // returns the distance from our initial start location
    fn distance(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

struct Walker {
    journey  : Vec<Location>,
    heading  : Heading,
}

impl Walker {
    pub fn new() -> Walker {
        return Walker {
            journey:  vec![Location::initial()], 
            heading:  Heading::North,
        };
    }

    pub fn walk(&mut self, instruction: &String) {
        // first grab the new direction and the number of blocks
        let (new_heading, blocks) = instruction.split_at(1);

        // update the current heading
        self.heading = self.parse_new_heading(new_heading.chars().nth(0).unwrap());

        // parse blocks as int
        let blocks_as_i32 : i32 = blocks.parse().unwrap();

        // lastly add the new location to our journey
        for _ in 0..blocks_as_i32 {
            let step = self.step();
            self.journey.push(step);
        }
    }

    // return the sum of X's and Y's absolute values
    pub fn total_distance(&self) -> i32 {
        match self.journey.last() {
            Some(last) => last.distance(),
            None => 0,
        }
    }

    // this method returns a vector with locations visited twice or more
    pub fn calculate_revisits(&self) -> Vec<&Location>{
        let mut revisits = Vec::new();

        // iterate over each location, and check if they appear again
        // but with another index (other point in our journey)
        for (index_a, location_a) in self.journey.iter().enumerate() {
            for (index_b, location_b) in self.journey.iter().enumerate() {
                if index_a != index_b && location_a == location_b {
                    revisits.push(location_b);
                }
            }
        }

        return revisits;
    }

    fn parse_new_heading(&self, new_heading : char) -> Heading {
        match self.heading {
            Heading::North => {
                match new_heading {
                    'R' => Heading::East,
                    'L' => Heading::West,
                     _   => Heading::North,
                }
            },

            Heading::East => {
                match new_heading {
                    'R' => Heading::South,
                    'L' => Heading::North,
                     _   => Heading::North,
                }
            },

            Heading::South => {
                match new_heading {
                    'R' => Heading::West,
                    'L' => Heading::East,
                     _   => Heading::North,
                }
            },

            Heading::West => {
                match new_heading {
                    'R' => Heading::North,
                    'L' => Heading::South,
                     _   => Heading::North,
                }
            }
        }
    }

    fn step(&self) -> Location {
        let last_known_location = self.journey.last().unwrap();
        return match self.heading {
            Heading::North => {
                Location {
                    y: last_known_location.y + 1,
                    x: last_known_location.x,
                }
            },

            Heading::East => {
                Location {
                    x: last_known_location.x + 1,
                    y: last_known_location.y,
                }
            },

            Heading::South => {
                Location {
                    y: last_known_location.y - 1,
                    x: last_known_location.x,
                }
            },

            Heading::West => {
                Location {
                    x: last_known_location.x - 1,
                    y: last_known_location.y,
                }
            }
        };
    }
}

fn main() {

    // grab the given input directions
    let input = "R3, L5, R2, L2, R1, L3, R1, R3, L4, R3, L1, L1, R1, L3, R2, L3, L2, R1, R1, L1, R4, L1, L4, R3, L2, L2, R1, L1, R5, R4, R2, L5, L2, R5, R5, L2, R3, R1, R1, L3, R1, L4, L4, L190, L5, L2, R4, L5, R4, R5, L4, R1, R2, L5, R50, L2, R1, R73, R1, L2, R191, R2, L4, R1, L5, L5, R5, L3, L5, L4, R4, R5, L4, R4, R4, R5, L2, L5, R3, L4, L4, L5, R2, R2, R2, R4, L3, R4, R5, L3, R5, L2, R3, L1, R2, R2, L3, L1, R5, L3, L5, R2, R4, R1, L1, L5, R3, R2, L3, L4, L5, L1, R3, L5, L2, R2, L3, L4, L1, R1, R4, R2, R2, R4, R2, R2, L3, L3, L4, R4, L4, L4, R1, L4, L4, R1, L2, R5, R2, R3, R3, L2, L5, R3, L3, R5, L2, R3, R2, L4, L3, L1, R2, L2, L3, L5, R3, L1, L3, L4, L3";

    // parse directions as single command unit by splitting them on the comma
    let directions: Vec<String> = input.split(", ")
        .map(|v| v.to_string())
        .collect();

    // initialize a new walker, starting at relative x 0, y 0 facing north
    let mut walker = Walker::new();

    // start processing directions
    for direction in &directions {
        walker.walk(direction);
    }

    // what is the total distance from last location to initial location
    println!("total distance: {:?}", walker.total_distance());

    let revisits      = walker.calculate_revisits();
    let first_revisit = revisits.first().unwrap();
    println!("first revisit is {} blocks from base", first_revisit.distance());

}
