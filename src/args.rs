use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CsgoItemArgs {
    /// Input txt file
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Output csv file to destination
    #[arg(short, long, required = false)]
    pub output: Option<PathBuf>,

    /// Print out in table
    #[arg(short, long, action)]
    pub table: bool,
}
