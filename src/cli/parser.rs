use super::args::{ArgumentParser, ParsedArguments};
use crate::file_io::validate_lua_file;
use clap::{Arg, Command};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliArgs {
    pub script: PathBuf,
    pub script_args: Vec<String>,
    pub parsed_args: ParsedArguments,
    pub verbose: bool,
    pub debug: bool,
    pub timeout: Option<u64>,
    pub show_help: bool,
    pub module: Option<String>,
}

pub fn build_cli() -> Command {
    Command::new("hype")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("A high-performance Lua scripting engine with CLI interface")
        .long_about(
            "Hype-rs is a fast, efficient Lua runtime environment that allows you to execute Lua scripts \
             with various options for debugging, verbose output, and timeout control. It provides a clean \
             command-line interface for running Lua scripts with additional arguments."
        )
        .arg(
            Arg::new("script")
                .help("The Lua script file to execute")
                .required_unless_present_any(&["help_args", "module"])
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("script_args")
                .help("Arguments to pass to the Lua script (supports --key=value, --flag, and positional args)")
                .value_parser(clap::value_parser!(String))
                .num_args(0..)
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
        )
        .arg(
            Arg::new("help_args")
                .long("help-args")
                .help("Show help for script argument parsing")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Enable debug information")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .help("Script execution timeout in seconds")
                .value_name("SECONDS")
                .value_parser(clap::value_parser!(u64))
        )
        .arg(
            Arg::new("module")
                .short('m')
                .long("module")
                .help("Load and execute a Lua module")
                .value_name("PATH")
        )

        .after_help(
            "Examples:\n\
   hype script.lua                    # Run a Lua script\n\
   hype script.lua arg1 arg2          # Run with positional arguments\n\
   hype script.lua --name=John arg1    # Run with named arguments\n\
   hype script.lua --verbose --debug   # Run with flags\n\
   hype script.lua --timeout 30       # Run with 30s timeout\n\
   hype --module ./my-module.lua      # Load and execute a Lua module\n\
   hype -m app.lua                    # Short form of --module\n\
   hype script.lua --help-args         # Show argument parsing help\n\
   hype --version                     # Show version\n\
   hype --help                        # Show this help"
        )
}

pub fn parse_args() -> Result<CliArgs, String> {
    let matches = build_cli()
        .try_get_matches()
        .map_err(|e| format!("Argument parsing error: {}", e))?;

    let show_help = matches.get_flag("help_args");
    let verbose = matches.get_flag("verbose");
    let debug = matches.get_flag("debug");
    let timeout = matches.get_one::<u64>("timeout").copied();
    let module = matches.get_one::<String>("module").cloned();

    // If help is requested, we can skip script validation
    if show_help {
        // Create a dummy script path for help display
        let script = PathBuf::from("script.lua");
        let script_args = Vec::new();
        let arg_parser = ArgumentParser::new();
        let parsed_args = arg_parser.parse(&script, &script_args);

        return Ok(CliArgs {
            script,
            script_args,
            parsed_args,
            verbose,
            debug,
            timeout,
            show_help,
            module,
        });
    }

    // Get the script file path - optional if module is provided
    let script = if let Some(ref module_path) = module {
        // When using module, we don't need a script file
        PathBuf::from(module_path)
    } else {
        let script_path = matches
            .get_one::<PathBuf>("script")
            .ok_or("Script file is required unless using --module")?
            .clone();

        // Validate the script file using our robust validator
        if let Err(e) = validate_lua_file(&script_path) {
            return Err(format!("Script validation failed: {}", e));
        }
        script_path
    };

    // Get script arguments (everything after --)
    let script_args = matches
        .get_many::<String>("script_args")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<String>>();

    // Parse script arguments using the new argument parser
    let arg_parser = ArgumentParser::new();
    let parsed_args = arg_parser.parse(&script, &script_args);

    Ok(CliArgs {
        script,
        script_args,
        parsed_args,
        verbose,
        debug,
        timeout,
        show_help,
        module,
    })
}

pub fn validate_script_file(path: &PathBuf) -> Result<(), String> {
    // Use the new robust validator
    validate_lua_file(path).map_err(|e| e.to_string())
}
