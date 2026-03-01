mod assembler;
mod config;
mod constants;
mod processor;

use std::{
    env,
    fs::{read, read_to_string, write},
    path::Path,
};

use crate::config::Config;

fn start_up() {
    if let Err(error) = std::fs::create_dir_all(constants::BUILD_DIR) {
        panic!("Failed to create build directory: {}", error);
    }
}

fn parse_config() -> Config {
    dotenv::dotenv().ok().expect("Failed to load .env file");

    let text_model =
        env::var(constants::TEXT_MODEL_ENV).expect("TEXT_MODEL must be set in the .env file");
    let embedding_model = env::var(constants::EMBEDDING_MODEL_ENV)
        .expect("EMBEDDING_MODEL must be set in the .env file");
    let debug_build = env::var(constants::DEBUG_BUILD_ENV)
        .map(|value| value == "true")
        .unwrap_or(false);
    let debug_run = env::var(constants::DEBUG_RUN_ENV)
        .map(|value| value == "true")
        .unwrap_or(false);

    Config {
        text_model,
        embedding_model,
        debug_build,
        debug_run,
    }
}

fn build(file_path: &str, config: &Config) -> Result<(), String> {
    let source = read_to_string(file_path).map_err(|error| format!("Build failed: {}", error))?;
    let source: &'static str = Box::leak(Box::new(source));

    let mut compiler = assembler::Assembler::new(source);
    let byte_code = compiler
        .assemble()
        .map_err(|error| format!("Build failed: {}", error))?;

    if config.debug_build {
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
    write(&output_file_name, byte_code).map_err(|error| format!("Build failed: {}", error))?;

    println!("Build successful! Output written to {}", output_file_name);

    Ok(())
}

fn run(file_path: &str, config: &Config) -> Result<(), String> {
    let data = read(file_path).map_err(|error| format!("Run failed: {}", error))?;

    let mut processor = processor::Processor::new(config.clone());
    processor.load(data)?;
    processor.run();

    Ok(())
}

fn main() -> Result<(), String> {
    start_up();

    let config = parse_config();

    let args: Vec<String> = env::args().collect();
    let command = args
        .get(1)
        .ok_or_else(|| format!("No command provided. {}", constants::HELP_USAGE))?;
    let file_path = args
        .get(2)
        .ok_or_else(|| format!("No file path provided. {}", constants::HELP_USAGE))?;

    match command.as_str() {
        "build" => build(file_path, &config),
        "run" => run(file_path, &config),
        other => Err(format!("Unknown command: {}", other)),
    }
}
