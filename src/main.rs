mod assembler;
mod instruction;
mod opcode;
mod openai;
mod processor;
mod scanner;
mod token;

// use crate::{scanner::Scanner, token::TokenType};

use std::fs::read_to_string;

fn main() {
    let file_name = "src/example.aism";
    let instructions = match read_to_string(file_name) {
        Ok(content) => Box::leak(Box::new(content)).as_str(),
        Err(e) => panic!("Could not read file {}: {}", file_name, e),
    };

    // let mut scanner = Scanner::new(instructions);

    // loop {
    //     let token = scanner.scan_token();

    //     if token.token_type == TokenType::EOF {
    //         break;
    //     }

    //     let lexeme = &instructions[token.start..token.start + token.length];
    //     let message = match &token.error {
    //         Some(error) => error,
    //         None => "",
    //     };

    //     println!(
    //         "[Line {}] {:?} {} '{}'",
    //         token.line, token.token_type, message, lexeme
    //     );
    // }

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
