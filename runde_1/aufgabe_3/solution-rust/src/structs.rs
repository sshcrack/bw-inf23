use std::fmt::Display;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlockType {
    Row,
    Column,
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::Row => write!(f, "Row"),
            BlockType::Column => write!(f, "Column"),
        }
    }
}

pub enum AlgorithmStage {
    SwapBlocks,
    SwapSingle,
    SwapNumbers,
}