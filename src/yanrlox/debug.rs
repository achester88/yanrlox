use crate::yanrlox::chunk::{Chunk, Opcode};

pub fn disassembleChunk(chunk: &Chunk) {
    println!("#### DEBUG START ####");
    let mut i = 0;
    while i < chunk.code.len() {
        let op = chunk.code[i];
        print!("[{}] ", i);
        match op {
            0x00 => print!("Return<{}>", chunk.get_line(i)+1),
            0x01 => { //Constant
                i += 1;
                print!("Constant<{}>; [{}] {}(@{})", chunk.get_line(i)+1, i, chunk.constants[chunk.code[i] as usize], chunk.code[i]);
            }
            0x02 => {
                i += 1;
                print!("Negate<{}>; [{}] {}(@{})", chunk.get_line(i)+1, i, chunk.constants[chunk.code[i] as usize], chunk.code[i]);
            },
            0x03 => print!("Add"),
            0x04 => print!("Subtract"),
            0x05 => print!("Multiply"),
            0x06 => print!("Divide"),
            _ => print!("\x1b[91mError\x1b[0m: UNKNOW OPCODE: {:#04x?}", op)
        };

        println!("");
        i += 1;
    }
    println!("#### DEBUG  END  ####");
}


//int disassembleInstruction(Chunk* chunk, int offset);


//(&chunk, "test chunk");