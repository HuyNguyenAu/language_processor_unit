mod assembler;
mod constants;
mod processor;

use std::{
    env,
    fs::{read, read_to_string, write},
    path::Path,
};

fn build(file_path: &str, debug: bool) {
    let source = match read_to_string(file_path) {
        Ok(content) => Box::leak(Box::new(content)).as_str(),
        Err(error) => panic!("Build failed. Error: {}", error),
    };

    let mut compiler = assembler::Assembler::new(source);

    let byte_code = match compiler.assemble() {
        Ok(byte_code) => byte_code,
        Err(error) => panic!("Build failed. Error: {}", error),
    };

    if debug {
        println!("Assembled byte code ({} bytes):", byte_code.len());

        // Print every 4 bytes as a single instruction
        for (chuck_index, byte) in byte_code.chunks(4).enumerate() {
            let index = chuck_index * 4;

            print!("{} {:02X} ({}): ", chuck_index, index, index);
            println!("{:?} ", byte);
        }

        println!();
    }

    let path = Path::new(file_path);
    let file_stem = match path.file_stem() {
        Some(value) => value,
        None => panic!("Build failed. Error: Could not determine file stem"),
    };
    let output_file_name = match file_stem.to_str() {
        Some(value) => format!("{}/{}.caism", constants::BUILD_DIR, value),
        None => panic!("Build failed. Error: Could not convert file stem to string"),
    };

    match write(&output_file_name, byte_code) {
        Ok(_) => println!("Build successful! Output written to {}", &output_file_name),
        Err(error) => panic!("Build failed. Error: {}", error),
    }
}

fn run(file_path: &str, debug: bool) {
    let data = match read(file_path) {
        Ok(value) => value,
        Err(error) => panic!("Run failed. Error: {}", error),
    };

    let mut processor = processor::Processor::new();

    processor.load(data);
    processor.run(debug);
}

fn startup() {
    if !Path::new(constants::BUILD_DIR).exists()
        && let Err(error) = std::fs::create_dir_all(constants::BUILD_DIR)
    {
        panic!("Failed to create build directory. Error: {}", error);
    }
}

fn main() {
    startup();

    let args: Vec<String> = env::args().collect();
    let command = match args.get(1) {
        Some(value) => value,
        None => panic!("No command provided"),
    };
    let file_path = match args.get(2) {
        Some(value) => value,
        None => panic!("No file path provided"),
    };
    let debug = args.get(3).map_or(false, |arg| arg == "--debug");

    match command.as_str() {
        "build" => build(file_path, debug),
        "run" => run(file_path, debug),
        _ => panic!("Unknown command: {}", command),
    }
}
