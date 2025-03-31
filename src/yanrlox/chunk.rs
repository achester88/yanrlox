#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Constant,
    Return
}

type Value = f32;

pub struct Chunk {
    pub code: Vec<i16>,
    pub constants: Vec<Value>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk{ code: vec![], constants: vec![] }
    }

    pub fn add_constant(&mut self, val: Value) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }
}