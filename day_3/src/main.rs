use std::io::prelude::*;
use std::fs::File;

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

    fn from_row(dim : &str) -> Triangle {
        let dim : Vec<isize> = dim.split(' ')
            .filter(|dim| { dim.len() > 0})
            .map(|dim| { dim.parse().unwrap() })
            .collect();

        Triangle { 
            a: dim.get(0).unwrap().to_owned(), 
            b: dim.get(1).unwrap().to_owned(), 
            c: dim.get(2).unwrap().to_owned(), 
        }
    }
}

fn main() {
    // read triangles from input
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();


    // Part A
    let valid_triangles : Vec<Triangle> = input.split('\n')
        .map(|dim| {Triangle::from_row(dim)})
        .filter(|triangle| { triangle.is_valid()})
        .collect();

    println!("Valid triangles for part A: {}", valid_triangles.len());

}
