use std::io::{Read, Write};

use crate::args::Args;

pub fn run<R: Read, W: Write>(reader: R, mut writer: W, args: Args) {
    todo!("Write the program logic here");
}

#[cfg(test)]
mod tests {}
