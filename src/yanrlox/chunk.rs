#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Return = 0,
    Constant = 1,
}

type Value = f32;

pub struct Chunk {
    pub code: Vec<i16>,
    pub constants: Vec<Value>,
    pub line_count: Vec<i8>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk{ code: vec![], constants: vec![], line_count: vec![] }
    }

    pub fn add_constant(&mut self, val: Value) -> i16 {
        self.constants.push(val);
        (self.constants.len() - 1) as i16
    }

    pub fn push_op(&mut self, opcode: Opcode, line: usize) {
        self.code.push(opcode as i16);
        self.line_push(line);
    }

    pub fn push_i16(&mut self, val: i16, line: usize) {
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