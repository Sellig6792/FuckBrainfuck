use clap::Parser;
use colored::Colorize;

mod cli;
mod fuckbf;
mod update;
mod version;

use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Before running help or version, delete the old file
    update::delete_old_file();

    let args = Cli::parse();

    if args.update || args.update_force {
        update::update(args.update_force)?;
    } else if let Some(path) = args.path {
        cli::run(&path, args.optimize)?;
    } else {
        println!("{} The following required arguments were not provided:\n  {}\n\n{} {} [OPTIONS] [PATH]\n\nFor more information try {}",
                 "error:".red().bold(),
                 "PATH".green(),
                 "Usage:".bold().underline(),
                 "fuckbf.exe".bold(),
                 "'--help'".bold()
        );
    }
    Ok(())
}
