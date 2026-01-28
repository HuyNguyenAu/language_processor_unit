mod assembler;
mod instruction;
mod microcode;
mod opcode;
mod openai;
mod processor;
mod scanner;
mod token;

use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide the path to the assembly file.");
    }

    let file_name = &args[1];
    let instructions = match read_to_string(file_name) {
        Ok(content) => Box::leak(Box::new(content)).as_str(),
        Err(e) => panic!("Could not read file {}: {}", file_name, e),
    };

    let mut compiler = assembler::Assembler::new(instructions);

    let byte_code = match compiler.assemble() {
        Ok(byte_code) => byte_code,
        Err(e) => panic!("Assembly error: {}", e),
    };

    println!("Assembled bytecode: {:02X?}", byte_code);

    let mut processor = processor::Processor::new();

    processor.load_bytecode(byte_code);
    processor.run();
}
