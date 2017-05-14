use std;

#[derive(Debug, PartialEq)]
pub struct Dimension {
    pub x: usize,
    pub y: usize,
}

impl std::str::FromStr for Dimension {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dimensions = s.split('x');
        Ok(Dimension {
            x: dimensions.next().unwrap().parse().unwrap(),
            y: dimensions.next().unwrap().parse().unwrap(),
        })
    }
}
