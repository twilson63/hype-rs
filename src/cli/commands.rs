use crate::cli::args::ArgumentParser;
use crate::cli::install::{
    install_package, list_packages, uninstall_package, which_command, InstallArgs,
};
use crate::cli::parser::CliArgs;
use crate::engine::{ExecutionConfig, ExecutionEngine, ExecutionResult, OutputFormat};
use crate::error::HypeError;
use crate::lua::module_env::create_module_env;
use crate::lua::require::setup_require_fn;
use crate::lua::{create_cli_config, LuaStateManager};
use crate::modules::loader::ModuleLoader;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn run_script(args: CliArgs) -> Result<(), HypeError> {
    // Show argument parsing help if requested
    if args.show_help {
        let parser = ArgumentParser::new();
        let help = parser.generate_help(
            &args.parsed_args.script_name,
            Some("Execute Lua scripts with comprehensive argument support"),
        );
        println!("{}", help);
        return Ok(());
    }

    if args.verbose {
        eprintln!("Executing script: {}", args.script.display());
        if let Some(ref module_path) = args.module {
            eprintln!("Module mode enabled: {}", module_path);
        }
        if !args.script_args.is_empty() {
            eprintln!("Script arguments: {:?}", args.script_args);
            eprintln!("Parsed arguments:");
            eprintln!("  Indexed: {:?}", args.parsed_args.indexed_args);
            eprintln!("  Named: {:?}", args.parsed_args.named_args);
            eprintln!("  Flags: {:?}", args.parsed_args.flags);
        }
    }

    if args.debug {
        eprintln!("Debug mode enabled");
        eprintln!(
            "Timeout: {:?}",
            args.timeout.map(|t| format!("{} seconds", t))
        );
        eprintln!("Argument parsing enabled");
    }

    if let Some(ref module_path) = args.module {
        return run_module(module_path.clone(), args);
    }

    // Create execution configuration
    let mut config = ExecutionConfig::default();
    config.script_path = args.script.clone();
    config.script_args = args.script_args.clone();
    config.parsed_args = Some(args.parsed_args.clone());
    config.verbose = args.verbose;
    config.debug = args.debug;
    config.timeout = args.timeout.map(Duration::from_secs);
    config.capture_output = true;
    config.output_format = OutputFormat::Text;
    config.enable_stats = args.verbose || args.debug;
    config.allow_debug_operations = args.debug;
    config.allow_file_operations = args.debug;
    config.allow_os_operations = args.debug;
    config.allow_package_loading = args.debug;

    // Create and run execution engine
    let mut engine = ExecutionEngine::new(config)?;
    let result = engine.execute()?;

    // Handle execution result
    handle_execution_result(result, args.verbose)?;

    Ok(())
}

fn run_module(module_path: String, args: CliArgs) -> Result<(), HypeError> {
    let path = Path::new(&module_path);

    if !path.exists() {
        return Err(HypeError::File(crate::error::FileError::NotFound(
            path.to_path_buf(),
        )));
    }

    let module_code = fs::read_to_string(path)
        .map_err(|_| HypeError::File(crate::error::FileError::NotFound(path.to_path_buf())))?;

    let mut lua_config = create_cli_config(
        args.verbose,
        args.debug,
        args.timeout.map(Duration::from_secs),
    );
    lua_config.allow_file_operations = args.debug;
    lua_config.allow_os_operations = args.debug;
    lua_config.allow_debug_operations = args.debug;
    lua_config.allow_package_loading = args.debug;

    let state_manager = LuaStateManager::new(lua_config)?;
    let lua = state_manager.lua.lock().unwrap();

    let cwd = std::env::current_dir().map_err(|e| HypeError::Io(e))?;
    let loader = Arc::new(Mutex::new(ModuleLoader::new(cwd)));

    setup_require_fn(&lua, loader)
        .map_err(|e| HypeError::Lua(format!("Failed to setup module system: {}", e)))?;

    let env = create_module_env(&lua, path)
        .map_err(|e| HypeError::Lua(format!("Failed to create module environment: {}", e)))?;

    let globals = lua.globals();
    if let Ok(require_fn) = globals.get::<_, mlua::Value>("require") {
        env.set("require", require_fn)
            .map_err(|e| HypeError::Lua(format!("Failed to set require function: {}", e)))?;
    }

    let chunk = lua.load(&module_code).set_environment(env);

    chunk
        .eval::<()>()
        .map_err(|e| HypeError::Lua(format!("Failed to execute module: {}", e)))?;

    if args.verbose {
        eprintln!("Module executed successfully");
    }

    Ok(())
}

fn handle_execution_result(result: ExecutionResult, verbose: bool) -> Result<(), HypeError> {
    if result.success {
        // Print stdout if we have any output
        if !result.output.is_empty() {
            print!("{}", result.output);
        }

        if verbose {
            eprintln!("Script execution completed successfully");
            eprintln!("Execution time: {:?}", result.execution_time);

            if let Some(stats) = result.stats {
                eprintln!("Statistics collected:");
                eprintln!(
                    "  Instructions executed: {}",
                    stats.lua_stats.instructions_executed
                );
                eprintln!("  Memory usage: {} bytes", stats.memory_usage.final_memory);
                eprintln!(
                    "  I/O operations: {} read, {} written",
                    stats.io_stats.bytes_read, stats.io_stats.bytes_written
                );
            }
        }
    } else {
        // Print error output
        if !result.error_output.is_empty() {
            eprint!("{}", result.error_output);
        }

        // Print stdout if we have any (might have partial output before error)
        if !result.output.is_empty() {
            print!("{}", result.output);
        }

        if verbose {
            eprintln!("Script execution failed");
            eprintln!("Exit code: {}", result.exit_code);
            eprintln!("Execution time: {:?}", result.execution_time);

            if let Some(error) = &result.error {
                eprintln!("Error: {}", error);
            }
        }

        return Err(HypeError::Execution(result.error_output));
    }

    Ok(())
}

pub fn show_version() {
    println!("hype version {}", env!("CARGO_PKG_VERSION"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
}

pub fn handle_install_command(
    path: Option<std::path::PathBuf>,
    force: bool,
    verbose: bool,
) -> Result<(), HypeError> {
    let args = InstallArgs {
        path,
        force,
        verbose,
    };
    install_package(args)
}

pub fn handle_uninstall_command(name: String, verbose: bool) -> Result<(), HypeError> {
    uninstall_package(name, verbose)
}

pub fn handle_list_command(json: bool, verbose: bool) -> Result<(), HypeError> {
    list_packages(verbose, json)
}

pub fn handle_which_command(command: String) -> Result<(), HypeError> {
    which_command(command)
}
