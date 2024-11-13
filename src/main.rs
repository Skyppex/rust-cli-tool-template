mod args;
mod io_utils;
mod path_utils;
mod program;

use std::io::Result;

use args::Args;
use clap::Parser;
use io_utils::*;

fn main() -> Result<()> {
    let args = Args::parse();

    let reader = get_reader(args.source.as_deref())?;
    let writer = get_writer(args.destination.as_deref())?;

    program::run(reader, writer, args);

    Ok(())
}
