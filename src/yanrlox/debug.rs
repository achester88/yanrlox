use crate::yanrlox::chunk::{Chunk, Opcode};

pub fn disassembleChunk(chunk: &Chunk) {
    println!("#### DEBUG START ####");
    let mut i = 0;
    while i < chunk.code.len() {
        let op = chunk.code[i];
        print!("[{}] ", i);
        match op {
            0 => print!("Return<{}>", chunk.get_line(i)+1),
            1 => { //Constant
                i += 1;
                print!("Constant<{}>; [{}] {}(@{})", chunk.get_line(i)+1, i, chunk.constants[chunk.code[i] as usize], chunk.code[i]);
            }
            _ => print!("{}. {}", i, op)
        };

        println!("");
        i += 1;
    }
    println!("#### DEBUG  END  ####");
}



//int disassembleInstruction(Chunk* chunk, int offset);


//(&chunk, "test chunk");