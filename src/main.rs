use std::{fs::read_to_string, path::PathBuf};
use clap::{Parser, Subcommand};
use kenken::KenKen;
use solve::solve;
use validate::Validator;

mod asg;
mod print;
mod validate;
mod kenken;
mod parse;
mod solve;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Solve { path ,verbose} => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            let sol = solve(&kenken);
            print::print(&kenken, sol, 10)?;
        },
        Commands::Print { path } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            print::print(&kenken, vec![], 10)?;
        },
        Commands::Validate { path } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            kenken.validate().unwrap();
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Solve {
        #[clap(parse(from_os_str))]
        path: PathBuf,

        #[clap(short, long)]
        verbose: bool
    },
    Validate {
        #[clap(parse(from_os_str))]
        path: PathBuf,
    },
    Print {
        #[clap(parse(from_os_str))]
        path: PathBuf,
    }
}


