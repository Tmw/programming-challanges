#[derive(Debug, PartialEq)]
pub enum Target {
    Row(usize),
    Column(usize),
}
