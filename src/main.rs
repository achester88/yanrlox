use std::env;
use std::io;
use std::process::exit;
use std::io::Write;
use std::env::args;

mod yanrlox;
use yanrlox::compiler::Compiler;
use yanrlox::vm::InterpretResult;

use crate::yanrlox::scanner;
use crate::yanrlox::error::*;

use crate::yanrlox::chunk::{Opcode, Chunk};
use crate::yanrlox::debug;
use crate::yanrlox::vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    
    if args.len() > 2 {
        println!("USAGE: yanrlox [source]");
        exit(64);
    } else if args.len() == 2 {
        run_program(&args[1]);
    } else {
        run_prompt();
    }

}

pub fn interpret(source: &str) -> Result<(), Error> {
    let chunk: Chunk;

    let mut comp = Compiler::new(source);

    match comp.compile() {
        Ok(c) => chunk = c,
        Err(e) => return Err(e)
    }

    let mut virm = vm::Vm::new(chunk, true);

    return virm.run();
}

fn run_program(location: &String) {
    let source: String;
    match std::fs::read_to_string(&location) {
        Ok(x) => source = x,
        Err(_e) => {
            println!("\x1b[91mError\x1b[0m: Could Not Find File at Path: `{}`", location);
            exit(1);
        }
    }

    match interpret(&source) {
        Ok(()) => exit(0),
        Err(error) => {
            throw_error(source, error);
            exit(64);
        }
    }
}

fn run_prompt() {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if &input == "exit\n" {
            exit(0);
        } else {
            match interpret(&input) {
                Err(error) => throw_error(input, error),
                _ => {}
            }
        }

    }
}