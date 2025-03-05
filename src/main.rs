use std::env;
use std::panic;
use std::io;
use std::process::exit;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("USAGE: yanrlox [source]");
        exit(64);
    } else if args.len() == 2 {
        runProgram(&args[1]);
    } else {
        runPrompt();
    }
    
}

fn run(input: String) {
    println!("NOW RUNNING\n\n|{}|", input);
}

fn runProgram(location: &String) {
    let mut source: String;
    match std::fs::read_to_string(&location) {
        Ok(x) => source = x,
        Err(e) => {
            println!("\x1b[91mError\x1b[0m: Could Not Find File at Path: `{}`", location);
            exit(1);
        }
    }
    run(source);
}

fn runPrompt() {
    let mut cont = true;
    while cont {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if &input == "exit\n" {
            exit(0);
        } else {
            run(input.clone());
        }

    }
}
