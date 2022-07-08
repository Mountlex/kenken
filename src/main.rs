use clap::{Parser, Subcommand};
use draw::DEFAULT_CONFIG;
use kenken::KenKen;
use parse::parse;
use solve::solve;
use std::{fs::read_to_string, path::PathBuf};
use validate::Validator;

mod asg;
mod draw;
mod kenken;
mod gen;
mod parse;
mod print;
mod solve;
mod validate;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Solve { path, verbose } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            let sol = solve(&kenken);
            print::print(&kenken, sol, 10)?;
        }
        Commands::Generate { size } => {
            let kenken = gen::generate(size);
            let sol = solve(&kenken);
            print::print(&kenken, sol, 10)?;
        }
        Commands::Print { path } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            print::print(&kenken, vec![], 10)?;
        }
        Commands::Draw { path } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            draw::draw(&kenken, &draw::DEFAULT_CONFIG)?;
        }
        Commands::Validate { path } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            kenken.validate().unwrap();
        }
        Commands::Save { input, output } => {
            let game = parse(&input)?;
            let content = ron::to_string(&game)?;
            if let Some(path) = output {
                std::fs::write(path, content)?;
            } else {
                std::fs::write(format!("kenken{}.ron", game.id), content)?;
            }
            print::print(&game, vec![], 10)?;
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Solve {
        #[clap(parse(from_os_str))]
        path: PathBuf,

        #[clap(short, long)]
        verbose: bool,
    },
    Generate {
        size: u16,
    },
    Validate {
        #[clap(parse(from_os_str))]
        path: PathBuf,
    },
    Print {
        #[clap(parse(from_os_str))]
        path: PathBuf,
    },
    Draw {
        #[clap(parse(from_os_str))]
        path: PathBuf,
    },
    Save {
        input: String,

        #[clap(parse(from_os_str))]
        output: Option<PathBuf>,
    },
}
