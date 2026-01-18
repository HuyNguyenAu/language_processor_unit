mod assembler;
mod instruction;
mod scanner;
mod token;

use crate::{scanner::Scanner, token::TokenType};

use std::fs::read_to_string;

fn main() {
    let file_name = "src/example.aism";
    let instructions = match read_to_string(file_name) {
        Ok(content) => Box::leak(Box::new(content)).as_str(),
        Err(e) => panic!("Could not read file {}: {}", file_name, e),
    };

    let mut scanner = Scanner::new(instructions);

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

    if compiler.assemble() {
        println!("Compilation succeeded.");
        println!("Bytecode: {:02X?}", compiler.byte_code);
    } else {
        println!("Compilation failed.");
    }
}
