/*
Now that you can think clearly, you move deeper into the labyrinth of hallways and office furniture that makes up this part of Easter Bunny HQ. This must be a graphic design department; the walls are covered in specifications for triangles.

Or are they?

The design document gives the side lengths of each triangle it describes, but... 5 10 25? Some of these aren't triangles. You can't help but mark the impossible ones.

In a valid triangle, the sum of any two sides must be larger than the remaining side. For example, the "triangle" given above is impossible, because 5 + 10 is not larger than 25.

In your puzzle input, how many of the listed triangles are possible?

--- Part Two ---

Now that you've helpfully marked up their design documents, it occurs to you that triangles are specified in groups of three vertically. Each set of three numbers in a column specifies a triangle. Rows are unrelated.

For example, given the following specification, numbers with the same hundreds digit would be part of the same triangle:

101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603
In your puzzle input, and instead reading by columns, how many of the listed triangles are possible?

*/

use std::io::prelude::*;
use std::fs::File;

type Dimensions = (isize, isize, isize);

struct Triangle {
    a : isize,
    b : isize,
    c : isize,
}

impl Triangle {
    // triangle is only valid if sides A + B > C, B + C > A and C + A > B
    fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.b + self.c > self.a && self.c + self.a > self.b
    }

    // this method converts a row into dimensions
    fn row_to_dimensions(dim_str : &str) -> Dimensions {
        let dim : Vec<isize> = dim_str.split(' ')
            .filter(|dim| { !dim.is_empty()})
            .map(|dim| { dim.parse().unwrap() })
            .collect();

        (
            dim.get(0).unwrap().to_owned(), 
            dim.get(1).unwrap().to_owned(), 
            dim.get(2).unwrap().to_owned()
        ) as Dimensions
    }

    // this method actually generates a triangle from dimensions
    fn from_dimensions(dim : Dimensions) -> Triangle {
        Triangle { a: dim.0, b: dim.1, c: dim.2 }
    }
}

fn main() {
    // read triangles from input
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    // ---------------
    // Part A
    // ---------------
    let valid_triangles_part_a : Vec<Triangle> = input.lines()
        .map(|row| {Triangle::row_to_dimensions(row)})
        .map(|dim| {Triangle::from_dimensions(dim)})
        .filter(|triangle| {triangle.is_valid()})
        .collect();

    println!("Valid triangles for part A: {}", valid_triangles_part_a.len());
    // ---------------
    // Part B
    // ---------------

    // remove the newlines from the input
    let concatted_lines = input.replace('\n', "");

    // split in the whitespace characters this time, filter empties and parse as ints
    let dimensions_part_b : Vec<isize> = concatted_lines.split(' ')
        .filter(|entry| { !entry.is_empty()} )
        .map(|dim| { dim.parse().unwrap() })
        .collect();

    // the output of the above lines is as follows:
    // A1, A2, A3, B1, B2, B3, C1, C2, C3 where ABC are the sides, and 1-3 are
    // the various triangles. In order to make the creation of triangles easier
    // we'd prefer an input more like A1, B1, C1, A2, A2,.. etc so we can simply split
    // into chunks of three and have indexes 0-2, 3-5 etc the various sides. 
    let sorted_dimensions : Vec<isize> = dimensions_part_b.chunks(9)
        .flat_map(|chunk| {
            // move everything in the correct place
            // we could probably use swap for this, but meh ¯\_(ツ)_/¯
            return vec![
                chunk[0], chunk[3], chunk[6],
                chunk[1], chunk[4], chunk[7],
                chunk[2], chunk[5], chunk[8],
            ];

        })
        .collect();
    
    // nest step is to convert the sorted list of dimensions in chunks of three
    // into dimensions which can be passed to Triangle::from_dimensions
    // and filter the list so that only the valid ones stay
    let valid_triangles_part_b : Vec<Triangle> = sorted_dimensions.chunks(3)
        .map(|chunk| {
            // chunk is now three sides of the triangle as a slice.
            // lets convert this to a Dimensions type (tuple of three isize)
            (chunk[0], chunk[1], chunk[2]) as Dimensions
        })
        .map(|dim| {Triangle::from_dimensions(dim)})
        .filter(|triangle| {triangle.is_valid()})
        .collect();

    
    println!("Valid triangles for part B: {:?}", valid_triangles_part_b.len());


}
