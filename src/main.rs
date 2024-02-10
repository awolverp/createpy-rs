use colored::Colorize;
use std::process::ExitCode;

use makers::{call_reinitializer, MakersStructure};

mod arguments;
mod makers;

fn main() -> ExitCode {
    let args: arguments::Arguments = arguments::parse_args();

    let reinitialize_without_input = args.3.reinitialize_without_input.clone();

    let initializers: MakersStructure = match MakersStructure::try_from(args) {
        Ok(o) => o,
        Err(e) => {
            println!("{} {}", "error:".red().bold(), e);
            return ExitCode::FAILURE;
        }
    };
    // │
    println!("{}", "Creating project ...".bold());
    match call_reinitializer(&initializers.project, reinitialize_without_input) {
        Ok(_) => (),
        Err(e) => {
            println!("└── {} {}", "error:".red().bold(), e);
            return ExitCode::FAILURE;
        }
    }
    println!("└── {}", "END\n".green().bold());

    if let Some(git) = initializers.git {
        println!("{}", "Initializing git ...".bold());
        
        match call_reinitializer(&git, reinitialize_without_input) {
            Ok(_) => (),
            Err(e) => {
                println!("└── {} {}", "error:".red().bold(), e);
                return ExitCode::FAILURE;
            }
        }
        
        println!("└── {}", "END\n".green().bold());
    }

    if let Some(venv) = initializers.venv {
        println!("{}", "Creating virtual environment ...".bold());
        
        match call_reinitializer(&venv, reinitialize_without_input) {
            Ok(_) => (),
            Err(e) => {
                println!("└── {} {}", "error:".red().bold(), e);
                return ExitCode::FAILURE;
            }
        }
        
        println!("└── {}", "END\n".green().bold());
    }

    ExitCode::SUCCESS
}
