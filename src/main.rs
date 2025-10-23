mod cli;
mod config;
mod engine;
mod error;
mod file_io;
mod lua;
mod modules;

use cli::commands::run_script;
use cli::parser::parse_args;
use error::HypeError;

fn main() -> Result<(), HypeError> {
    // Parse command line arguments
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Run the script with the provided arguments
    run_script(args)
}
