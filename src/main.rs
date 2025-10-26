mod cli;
mod config;
mod engine;
mod error;
mod file_io;
mod lua;
mod modules;

use cli::commands::{
    handle_install_command, handle_list_command, handle_uninstall_command, handle_which_command,
    run_script,
};
use cli::parser::{parse_args, HypeCommand};
use error::HypeError;

fn main() -> Result<(), HypeError> {
    let command = match parse_args() {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match command {
        HypeCommand::Run(args) => run_script(args),
        HypeCommand::Install {
            path,
            force,
            verbose,
        } => handle_install_command(path, force, verbose),
        HypeCommand::Uninstall { name, verbose } => handle_uninstall_command(name, verbose),
        HypeCommand::List { json, verbose } => handle_list_command(json, verbose),
        HypeCommand::Which { command } => handle_which_command(command),
    }
}
