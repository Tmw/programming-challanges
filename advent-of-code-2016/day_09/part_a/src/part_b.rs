use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\((?P<length>[0-9]+)x(?P<times>[0-9]+)\)").unwrap();
}

pub fn solve(data: &str) {
    println!("{}", "------------- [ PART B ] -------------");
    println!("NOOT NOOT! its da sound of da pingu 🐧");
    println!("{}", "--------------------------------------");
}

#[derive(Debug)]
pub struct Node {
    content_length: usize,
    multiplier: usize,
    nodes: Vec<Node>
}

impl Node {
    pub fn len(&self) -> usize {
        self.multiplier * (self.content_length + self.nodes.iter().fold(0, {|v, n| v + n.len() }))
    }
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let cap = REGEX.captures_iter(s).nth(0).unwrap();

        // grab end of capture and grab new sequence from end of capture until end of s
        // parse new sequence into node and add onto nodes.

        let res = Node {
            content_length: cap["length"].parse().unwrap(),
            multiplier: cap["times"].parse().unwrap(),
            nodes: Vec::new(),
        };

        Ok(res)
    }
}

#[cfg(test)]
mod node_tests {
    use part_b::Node;

    #[test]
    fn test_solve_without_subnodes() {
        let subject = Node { content_length: 10, multiplier: 3, nodes: Vec::new() };
        assert_eq!(subject.len(), 30);
    }

    #[test]
    fn test_solve_with_one_level_subnodes() {
        // example: (22x11)(3x5)ICQ(9x5)IYUPTHPKX

        let subnodes = vec![
            Node { content_length: 3, multiplier: 5, nodes: Vec::new() },
            Node { content_length: 9, multiplier: 5, nodes: Vec::new() },
        ];

        let subject = Node {
            content_length: 0,
            multiplier: 22,
            nodes: subnodes
        };

        assert_eq!(subject.len(), 1320);
    }

    #[test]
    fn test_solve_with_multi_level_subnodes() {
        // example: (answer should be 5148)
        // (127x2)                          => (1098 + 140 + 1336) * 2 = 5148
        //      (41x6)                      => (144 + 39) * 6 = 1098
        //          (16x9)SIUZCKMFZFXKUYTQ  => 144
        //          (13x3)YBCVHJPPFAONV     => 39
        //      (10x14)BTRWBQRUHA           => 10 * 14 = 140
        //      (57x4)                      => (180 + 55 + 72 + 27) * 4 = 1336
        //          (12x15)ZUMPYOEOOBFW     => 180
        //          (5x11)YNLIJ             => 55
        //          (8x9)GBQFPTOH           => 72
        //          (9x3)GPFCSAPZD          => 27

        let subject = Node {
            content_length: 0,
            multiplier: 2,
            nodes: vec![
                Node {
                    content_length: 0,
                    multiplier: 6,
                    nodes: vec![
                        Node { content_length: 16, multiplier: 9, nodes: Vec::new() },
                        Node { content_length: 13, multiplier: 3, nodes: Vec::new() },
                    ]
                },

                Node {
                    content_length: 10,
                    multiplier: 14,
                    nodes: Vec::new()
                },

                Node {
                    content_length: 0,
                    multiplier: 4,
                    nodes: vec![
                        Node { content_length: 12, multiplier: 15, nodes: Vec::new() },
                        Node { content_length: 5, multiplier: 11, nodes: Vec::new() },
                        Node { content_length: 8, multiplier: 9, nodes: Vec::new() },
                        Node { content_length: 9, multiplier: 3, nodes: Vec::new() },
                    ]
                },
            ]
        };

        assert_eq!(subject.len(), 5148);
    }

    #[test]
    fn test_from_str_simple() {
        let node : Node = "(5x11)ABCDE".parse().unwrap();

        assert_eq!(node.content_length, 5);
        assert_eq!(node.multiplier, 11);
        assert_eq!(node.nodes.len(), 0);
    }

    #[test]
    fn test_from_str_with_subnodes() {
        let node : Node = "(10x4)(4x10)FGHI".parse().unwrap(); // total length should be 160

        // outer node
        assert_eq!(node.content_length, 0);
        assert_eq!(node.multiplier, 4);
        assert_eq!(node.nodes.len(), 1);

        println!("{:?}", node.nodes);

        // inner node
        let inner_node = node.nodes.first().unwrap();
        assert_eq!(inner_node.content_length, 4);
        assert_eq!(inner_node.multiplier, 10);
        assert!(inner_node.nodes.is_empty());
    }

    #[test]
    fn test_from_str_with_nested_and_peers() {
        // insert headaches here.
    }
}
