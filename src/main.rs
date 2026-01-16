mod compiler;
mod scanner;
mod token;

use crate::scanner::Scanner;
use crate::token::TokenType;

use std::fs::read_to_string;

// enum OPCode {
//     VAR,

//     ADD,
//     SUB,

//     SIM,

//     BGE,

//     JMP,
//     STOP,
// }

// impl OPCode {
//     fn as_str(&self) -> &'static str {
//         match self {
//             OPCode::VAR => "VAR",

//             OPCode::ADD => "ADD",
//             OPCode::SUB => "SUB",

//             OPCode::SIM => "SIM",

//             OPCode::BGE => "BGE",

//             OPCode::JMP => "JMP",
//             OPCode::STOP => "STOP",
//         }
//     }
// }

// enum Operand {
//     Number(f32),
//     Text(&'static str),
// }

// struct Instruction {
//     code: OPCode,
//     operand_1: Option<Operand>,
//     operand_2: Option<Operand>,
//     operand_3: Option<Operand>,
// }

fn main() {
    let file_name = "src/example.lisa";
    let instructions = match read_to_string(file_name) {
        Ok(content) => Box::leak(Box::new(content)),
        Err(e) => panic!("Could not read file {}: {}", file_name, e),
    };

    let mut scanner = Scanner::new(instructions);

    loop {
        let token = scanner.scan_token();

        println!(
            "[Line {}] {:?} {}",
            token.line,
            token.token_type,
            &instructions[token.start..token.start + token.length]
        );

        if token.token_type == TokenType::EOF {
            return;
        }
    }
}
