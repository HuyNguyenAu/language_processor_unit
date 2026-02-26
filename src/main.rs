mod assembler;
mod constants;
mod processor;

use std::{
    env,
    fs::{read, read_to_string, write},
    path::Path,
};

fn build(file_path: &str, debug: bool) -> Result<(), String> {
    let source = read_to_string(file_path).map_err(|e| format!("Build failed: {}", e))?;
    let source: &'static str = Box::leak(Box::new(source));

    let mut compiler = assembler::Assembler::new(source);
    let byte_code = compiler
        .assemble()
        .map_err(|e| format!("Build failed: {}", e))?;

    if debug {
        println!("Assembled byte code ({} bytes):", byte_code.len());
        for (chunk_idx, chunk) in byte_code.chunks(4).enumerate() {
            let index = chunk_idx * 4;
            println!("{} {:02X} ({}): {:?}", chunk_idx, index, index, chunk);
        }
        println!();
    }

    let path = Path::new(file_path);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Build failed: could not determine output filename".to_string())?;

    let output_file_name = format!("{}/{}.lpu", constants::BUILD_DIR, stem);
    write(&output_file_name, byte_code).map_err(|e| format!("Build failed: {}", e))?;

    println!("Build successful! Output written to {}", output_file_name);
    Ok(())
}

fn run(file_path: &str, debug: bool) -> Result<(), String> {
    let data = read(file_path).map_err(|e| format!("Run failed: {}", e))?;
    let mut processor = processor::Processor::new();
    processor.load(data)?;
    processor.run(debug);
    Ok(())
}

fn startup() {
    if let Err(error) = std::fs::create_dir_all(constants::BUILD_DIR) {
        panic!("Failed to create build directory: {}", error);
    }
}

fn main() -> Result<(), String> {
    startup();

    let args: Vec<String> = env::args().collect();
    let command = args
        .get(1)
        .ok_or_else(|| "No command provided".to_string())?;
    let file_path = args
        .get(2)
        .ok_or_else(|| "No file path provided".to_string())?;
    let debug = args.get(3).is_some_and(|arg| arg == "--debug");

    match command.as_str() {
        "build" => build(file_path, debug),
        "run" => run(file_path, debug),
        other => Err(format!("Unknown command: {}", other)),
    }
}
