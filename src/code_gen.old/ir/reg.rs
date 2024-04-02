use std::fmt;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reg(pub usize);

impl From<usize> for Reg {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
