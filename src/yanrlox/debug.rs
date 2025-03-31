use crate::yanrlox::chunk::{Chunk, Opcode};

pub fn disassembleChunk(chunk: &Chunk) {
    println!("#### START ####");
    for (op, i) in chunk.code.into_iter() {
        match op {
            _ => println!("{}. {}", i, op)
        };
    }
}



//int disassembleInstruction(Chunk* chunk, int offset);


//(&chunk, "test chunk");