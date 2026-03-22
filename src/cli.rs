use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cnlint")]
#[command(version)]
#[command(about = "Scan and clean Chinese comments in source code")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Check {
        path: String,
    },

    Scan {
        path: String,

        #[arg(long)]
        json: bool,
    },

    Clean {
        path: String,

        /// Clean line comments only.
        #[arg(long)]
        line: bool,

        /// Clean block comments only.
        #[arg(long)]
        block: bool,

        /// Collapse consecutive blank lines after cleaning.
        #[arg(long)]
        trim_empty: bool,

        /// Show matches without modifying files.
        #[arg(long)]
        dry_run: bool,

        /// Print results as JSON.
        #[arg(long)]
        json: bool,
    },
}
