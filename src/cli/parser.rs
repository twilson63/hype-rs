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

#[derive(Debug)]
pub enum HypeCommand {
    Run(CliArgs),
    Install {
        path: Option<PathBuf>,
        force: bool,
        verbose: bool,
    },
    Uninstall {
        name: String,
        verbose: bool,
    },
    List {
        json: bool,
        verbose: bool,
    },
    Which {
        command: String,
    },
    Agent,
}

pub fn build_cli() -> Command {
    let run_cmd = Command::new("run")
        .about("Run a Lua script (default command)")
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
        );

    let install_cmd = Command::new("install")
        .about("Install a package globally")
        .arg(
            Arg::new("path")
                .help("Path to package directory (defaults to current directory)")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force installation, overwriting existing package")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        );

    let uninstall_cmd = Command::new("uninstall")
        .about("Uninstall a globally installed package")
        .arg(
            Arg::new("name")
                .help("Name of the package to uninstall")
                .required(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        );

    let list_cmd = Command::new("list")
        .about("List all globally installed packages")
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output in JSON format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        );

    let which_cmd = Command::new("which")
        .about("Show which package provides a command")
        .arg(
            Arg::new("command")
                .help("Name of the command to lookup")
                .required(true),
        );

    let agent_cmd = Command::new("agent")
        .about("Output machine-readable documentation for LLM agents")
        .hide(true);

    Command::new("hype")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("A high-performance Lua scripting engine with CLI interface")
        .long_about(
            "Hype-rs is a fast, efficient Lua runtime environment that allows you to execute Lua scripts \
             with various options for debugging, verbose output, and timeout control. It also provides \
             package management for globally installing Lua-based CLI tools."
        )
        .subcommand(run_cmd)
        .subcommand(install_cmd)
        .subcommand(uninstall_cmd)
        .subcommand(list_cmd)
        .subcommand(which_cmd)
        .subcommand(agent_cmd)
        .arg(
            Arg::new("script")
                .help("The Lua script file to execute (backward compatibility)")
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("script_args")
                .help("Arguments to pass to the Lua script")
                .value_parser(clap::value_parser!(String))
                .num_args(0..)
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
                .global(true)
        )
        .after_help(
            "Examples:\n\
   hype script.lua arg1 arg2          # Run a Lua script (backward compatible)\n\
   hype run script.lua arg1 arg2      # Run with explicit subcommand\n\
   hype install                       # Install package from current directory\n\
   hype install ./my-package          # Install package from path\n\
   hype install --force               # Force reinstall\n\
   hype uninstall my-package          # Uninstall a package\n\
   hype list                          # List installed packages\n\
   hype list --json                   # List in JSON format\n\
   hype which mycli                   # Show which package provides 'mycli'\n\
   hype --version                     # Show version\n\
   hype --help                        # Show this help"
        )
}

pub fn parse_args() -> Result<HypeCommand, String> {
    let matches = build_cli()
        .try_get_matches()
        .map_err(|e| format!("Argument parsing error: {}", e))?;

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let cli_args = parse_run_args(sub_matches)?;
            Ok(HypeCommand::Run(cli_args))
        }
        Some(("install", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("path").cloned();
            let force = sub_matches.get_flag("force");
            let verbose = sub_matches.get_flag("verbose");
            Ok(HypeCommand::Install {
                path,
                force,
                verbose,
            })
        }
        Some(("uninstall", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>("name")
                .ok_or("Package name is required")?
                .clone();
            let verbose = sub_matches.get_flag("verbose");
            Ok(HypeCommand::Uninstall { name, verbose })
        }
        Some(("list", sub_matches)) => {
            let json = sub_matches.get_flag("json");
            let verbose = sub_matches.get_flag("verbose");
            Ok(HypeCommand::List { json, verbose })
        }
        Some(("which", sub_matches)) => {
            let command = sub_matches
                .get_one::<String>("command")
                .ok_or("Command name is required")?
                .clone();
            Ok(HypeCommand::Which { command })
        }
        Some(("agent", _)) => Ok(HypeCommand::Agent),
        None => {
            if let Some(_script_path) = matches.get_one::<PathBuf>("script") {
                let cli_args = parse_run_args_compat(&matches)?;
                Ok(HypeCommand::Run(cli_args))
            } else {
                Err("No command specified. Use 'hype --help' for usage information.".to_string())
            }
        }
        _ => Err("Unknown command".to_string()),
    }
}

fn parse_run_args(matches: &clap::ArgMatches) -> Result<CliArgs, String> {
    let show_help = if matches.contains_id("help_args") {
        matches.get_flag("help_args")
    } else {
        false
    };
    let verbose = matches.get_flag("verbose");
    let debug = if matches.contains_id("debug") {
        matches.get_flag("debug")
    } else {
        false
    };
    let timeout = if matches.contains_id("timeout") {
        matches.get_one::<u64>("timeout").copied()
    } else {
        None
    };
    let module = if matches.contains_id("module") {
        matches.get_one::<String>("module").cloned()
    } else {
        None
    };

    if show_help {
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

    let script = if let Some(ref module_path) = module {
        PathBuf::from(module_path)
    } else {
        let script_path = matches
            .get_one::<PathBuf>("script")
            .ok_or("Script file is required unless using --module")?
            .clone();

        if let Err(e) = validate_lua_file(&script_path) {
            return Err(format!("Script validation failed: {}", e));
        }
        script_path
    };

    let script_args = matches
        .get_many::<String>("script_args")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<String>>();

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

fn parse_run_args_compat(matches: &clap::ArgMatches) -> Result<CliArgs, String> {
    let verbose = matches.get_flag("verbose");

    let script_path = matches
        .get_one::<PathBuf>("script")
        .ok_or("Script file is required")?
        .clone();

    if let Err(e) = validate_lua_file(&script_path) {
        return Err(format!("Script validation failed: {}", e));
    }

    let script_args = matches
        .get_many::<String>("script_args")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<String>>();

    let arg_parser = ArgumentParser::new();
    let parsed_args = arg_parser.parse(&script_path, &script_args);

    Ok(CliArgs {
        script: script_path,
        script_args,
        parsed_args,
        verbose,
        debug: false,
        timeout: None,
        show_help: false,
        module: None,
    })
}

pub fn validate_script_file(path: &PathBuf) -> Result<(), String> {
    validate_lua_file(path).map_err(|e| e.to_string())
}
