extern crate regex;
use regex::Regex;

fn main() {
    let data = include_str!("input.txt");
    let re = Regex::new(r"\((?P<length>[0-9]+)x(?P<times>[0-9]+)\)").unwrap();

    // find all markers and convert them to chunks
    let chunks: Vec<Chunk> = re.find_iter(data).map(|m| {
        let capture = re.captures(m.as_str()).unwrap();

        let length: usize = capture["length"].parse().unwrap();
        let repeat: usize = capture["times"].parse().unwrap();
        let sequence: &str = &data[m.end()..m.end() + length];

        Chunk {
            start: m.end(),
            end: m.end() + length,
            repeat: repeat,
            sequence: sequence.to_owned(),
        }

    }).collect();

    // filter out the chunks that are part of a previous chunk
    let mut valid_chunks: Vec<Chunk> = Vec::new();
    valid_chunks.push(chunks.first().unwrap().to_owned());

    for chunk in chunks.iter() {
        if chunk.start > valid_chunks.last().unwrap().end {
            valid_chunks.push(chunk.to_owned());
        }
    }

    // reconstruct the data but with all pieces filled in
    let mut output = String::new();

    for chunk in valid_chunks.iter() {
        output.push_str(chunk.expand().as_str());
    }

    println!("{}", "--------------------------------------");
    println!("uncompressed length: {}", output.len());
    println!("uncompressed value: {}", output);
    println!("{}", "--------------------------------------");

}

#[derive(Debug, Clone)]
struct Chunk {
    start: usize,
    end: usize,
    repeat: usize,
    sequence: String,
}

impl Chunk {
    fn expand(&self) -> String {
        let mut output = String::new();
        for _ in 0..self.repeat {
            output.push_str(&self.sequence)
        }

        output
    }
}

#[cfg(test)]
mod ChunkTests {
    use super::*;
    use ::Chunk;

    #[test]
    fn test_expand() {
        let subject = Chunk {
            start: 0,
            end: 10,
            repeat: 5,
            sequence: "EA".to_string(),
        };

        assert_eq!(subject.expand(), "EAEAEAEAEA");
    }
}
