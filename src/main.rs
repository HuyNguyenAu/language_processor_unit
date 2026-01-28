mod assembler;
mod processor;

use std::{
    env,
    fs::{read, read_to_string, write},
    path::Path,
};

fn build(file_path: &str) {
    let instructions = match read_to_string(file_path) {
        Ok(content) => Box::leak(Box::new(content)).as_str(),
        Err(error) => panic!("Build failed. Error: {}", error),
    };

    let mut compiler = assembler::Assembler::new(instructions);

    let byte_code = match compiler.assemble() {
        Ok(byte_code) => byte_code,
        Err(error) => panic!("Build failed. Error: {}", error),
    };

    let path = Path::new(file_path);
    let file_stem = match path.file_stem() {
        Some(value) => value,
        None => panic!("Build failed. Error: Could not determine file stem"),
    };
    let output_file_name = match file_stem.to_str() {
        Some(value) => format!("{}.caism", value),
        None => panic!("Build failed. Error: Could not convert file stem to string"),
    };

    match write(&output_file_name, byte_code) {
        Ok(_) => println!("Build successful! Output written to {}", &output_file_name),
        Err(error) => panic!("Build failed. Error: {}", error),
    }
}

fn run(file_path: &str) {
    let bytecode = match read(file_path) {
        Ok(value) => value,
        Err(error) => panic!("Run failed. Error: {}", error),
    };

    let mut processor = processor::Processor::new();

    processor.load(bytecode);
    processor.run();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = match args.get(1) {
        Some(value) => value,
        None => panic!("No command provided"),
    };
    let file_path = match args.get(2) {
        Some(value) => value,
        None => panic!("No file path provided"),
    };

    match command.as_str() {
        "build" => build(file_path),
        "run" => run(file_path),
        _ => panic!("Unknown command: {}", command),
    }
}
