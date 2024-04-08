use clap::Parser;

/// Write a concise description of the command here.
#[derive(Debug, Clone, Parser)]
#[command(version, author)]
pub struct Args {
    /// The source file to read from. If not provided, read from stdin.
    #[arg(short, long)]
    pub source: Option<String>,

    /// The destination file to write to. If not provided, write to stdout.
    #[arg(short, long)]
    pub destination: Option<String>,
}