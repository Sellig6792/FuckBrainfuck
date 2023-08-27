use std::{fs, path};

use anyhow::Context;
use clap::{arg, Parser};

use crate::fuckbf::{evaluation::Evaluator, *};

#[derive(Parser)]
#[clap(author, about, version)]
#[clap(propagate_version = true)]
pub struct Cli {
    // Path
    #[arg(required = false, help = "Path to the file to execute")]
    pub path: Option<path::PathBuf>,

    // Optimize flag (if path is given)
    #[arg(
        short = 'O',
        long = "optimize",
        help = "Optimize the code before executing it",
        default_value = "false"
    )]
    pub optimize: bool,

    // Update option (if no path is given)
    #[arg(short = 'U', long = "update", help = "Update the program")]
    pub update: bool,

    #[arg(long= "update-force", help="Force to install the last version from the main branch of the Github repository ", visible_aliases=["uf", "fu", "force-update"])]
    pub update_force: bool,
}

pub fn run(path: &path::PathBuf, optimize: bool) -> Result<(), Box<dyn std::error::Error>> {
    let program = fs::read_to_string(path)
        .with_context(|| format!("Could not read file: {}", path.display()))?;

    // Parse the program
    let mut parser = ast::Parser::new(program);
    let instructions = parser.parse();

    // Optimize the program if "optimize" is true
    if optimize {
        let mut optimizer = optimization::Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();

        // Evaluate the optimized program
        let mut brainfuck = Evaluator::new(optimized_instructions);
        brainfuck.evaluate(None, None);
    } else {
        // Evaluate the program
        let mut brainfuck = Evaluator::new(instructions);
        brainfuck.evaluate(None, None);
    }

    Ok(())
}
