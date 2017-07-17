use target::Target;

#[derive(Debug, PartialEq)]
pub struct Rotation {
    pub target: Target,
    pub rotation: usize,
}
