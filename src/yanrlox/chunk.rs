#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Return = 0x00,
    Constant = 0x01,
    Negate = 0x02,
    Add = 0x03,
    Subtract = 0x04,
    Multiply = 0x05,
    Divide = 0x06,
}

pub type Value = f32;

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub line_count: Vec<u8>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk{ code: vec![], constants: vec![], line_count: vec![] }
    }

    pub fn add_constant(&mut self, val: Value) -> u8 {
        self.constants.push(val);
        (self.constants.len() - 1) as u8
    }

    pub fn push_op(&mut self, opcode: Opcode, line: usize) {
        self.code.push(opcode as u8);
        self.line_push(line);
    }

    pub fn push_u8(&mut self, val: u8, line: usize) {
        self.code.push(val);
        self.line_push(line);
    }

    pub fn line_push(&mut self, line: usize) {
        if line < self.line_count.len() {
            self.line_count[line] += 1;
        } else {
            self.line_count.append(&mut vec![0; (line-self.line_count.len()) + 1]);
            self.line_count[line] += 1;
        }
    }

    pub fn get_line(&self, i: usize) -> usize {
        let mut total = 0;
        let mut l = 0;
        loop {
            total += self.line_count[l] as usize;

            if total > i { break };

            l += 1;
        }
        return l;
    }
}