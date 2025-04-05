use crate::yanrlox::chunk::{Chunk, Value};

#[macro_export]
macro_rules! binaryop {
    ( $stack:expr, $code:expr, $op:tt ) => {
        {
                let b = $stack.pop();
                let a = $stack.pop();

                $stack.push(a.unwrap() $op b.unwrap());
        }
    };
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError
}

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    debug: bool,
    stack: Vec<Value>
}

impl Vm {
    pub fn new(chunk: Chunk, debug: bool) -> Self {
        Vm {
            chunk: chunk,
            ip: 0,
            debug: debug,
            stack: vec![]
        }
    }

    pub fn run(&mut self) -> InterpretResult {

        loop {
            let current = self.chunk.code[self.ip];

            //#[cfg(feature = "complex")] 
            //{
                if self.debug {
                println!("      {:?}", self.stack);
                println!("-- {:#04x?}", current);
            }

            self.ip += 1;

            match current {
                0x00 => return {
                    println!("{}", self.stack.pop().unwrap_or(0.0));
                    InterpretResult::InterpretOk
                },
                0x01 => {
                    let val = self.read_costant();
                    self.stack.push(val);
                },
                0x02 => {
                    let val = -self.stack.pop().expect("Stack out of Bound at (-)");
                    self.stack.push(val);
                },
                0x03 => binaryop!(&mut self.stack, &mut self.chunk.code, +),
                0x04 => binaryop!(&mut self.stack, &mut self.chunk.code, -),
                0x05 => binaryop!(&mut self.stack, &mut self.chunk.code, *),
                0x06 => binaryop!(&mut self.stack, &mut self.chunk.code, /),
                _ => {}
            }
        }

    }

    pub fn read_costant(&mut self) -> Value {
        let index = self.chunk.code[self.ip as usize];
        self.ip += 1;
        self.chunk.constants[index as usize]
    }
}