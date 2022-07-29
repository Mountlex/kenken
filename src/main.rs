use anyhow::Result;
use clap::{Parser, Subcommand};
use gen::DifficultyConfig;
use kenken::KenKen;
use parse::parse;
use solve::solve;
use std::{fs::read_to_string, path::PathBuf};
use validate::Validator;
use std::fs;

mod asg;
mod draw;
mod gen;
mod kenken;
mod parse;
mod print;
mod solve;
mod validate;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Solve { path, verbose } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            let sol = solve(&kenken);
            print::print(&kenken, sol, 10)?;
        }
        Commands::Generate {
            size,
            add, sub, mul, div,
            size_factor,
        } => {
            // let mut wtr = csv::Writer::from_path("results.csv")?;
            // wtr.write_record(&["size", "type", "asgs"])?;
            // for size_f in 5..9 {
            //     for type_f in 1..10 {
            //         for _ in 0..10 {
            //             println!("{} {}", size_f, type_f);
            //             let size_factor = size_f as f32 / 10.0;
            //             let type_factor = type_f as f32 / 10.0;
            //             let kenken = gen::generate(size, &DifficultyConfig { size_factor, type_factor });
            //             let asgs = kenken.total_number_of_assignments();
            //             wtr.write_record(&[size_factor.to_string(), type_factor.to_string(), asgs.to_string()])?;
            //         }
            //     }
            // }

            // wtr.flush()?;

            let id = fs::read_dir("knkns")?.count() as u64 + 1;

            let gen_config = DifficultyConfig {
                size_factor,
                p_add: add,
                p_sub: sub,
                p_mul: mul,
                p_div: div,
            };
            let kenken = gen::generate(
                id,
                size,
                &gen_config,
            );

            let content = ron::to_string(&kenken)?;
            std::fs::write(format!("knkns_data/puzzle{}.ron", kenken.id), content)?;
            draw::draw(&kenken, &PathBuf::from(format!("knkns/puzzle{}.png", kenken.id)), &draw::DEFAULT_CONFIG, Some(&gen_config))?;
            
        }
        Commands::Print { path } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            print::print(&kenken, vec![], 10)?;
        }
        Commands::Draw { path } => {
            let input = read_to_string(path)?;
            let kenken: KenKen = ron::from_str(&input)?;
            //draw::draw(&kenken, "test.png".into(), &draw::DEFAULT_CONFIG)?;
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

        #[clap(short, long, default_value = "0.25")]
        add: f32,
        #[clap(short, long, default_value = "0.25")]
        sub: f32,
        #[clap(short, long, default_value = "0.25")]
        mul: f32,
        #[clap(short, long, default_value = "0.25")]
        div: f32,

        #[clap(long, default_value = "0.5")]
        size_factor: f32,
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
