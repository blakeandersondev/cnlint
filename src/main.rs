use anyhow::Result;
use clap::Parser;
use cnlint::{
    cleaner,
    cli::{Cli, Commands},
    scanner,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { path } => {
            let has_issue = scanner::check_path(&path)?;
            if has_issue {
                println!("Exists Chinese comment");
                std::process::exit(1);
            }
            println!("Not exists Chinese comment");
        }
        Commands::Scan { path, json } => {
            scanner::scan_path(&path, json)?;
        }
        Commands::Clean {
            path,
            line,
            block,
            trim_empty,
            dry_run,
            json,
        } => {
            cleaner::clean_path(&path, line, block, trim_empty, dry_run, json)?;
        }
    }

    Ok(())
}
