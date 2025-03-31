#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Return = 0,
    Constant = 1,
}

type Value = f32;

pub struct Chunk {
    pub code: Vec<i16>,
    pub constants: Vec<Value>,
    pub line: Vec<usize>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk{ code: vec![], constants: vec![], line: vec![] }
    }

    pub fn add_constant(&mut self, val: Value) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }

    pub fn push_op(&mut self, opcode: Opcode, line: usize) {
        self.code.push(opcode as i16);
        self.line.push(line);
    }
}