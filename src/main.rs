mod args;
mod program;

use std::{self, fs, io::{self, ErrorKind, Read, Result, Write}, path::{Path, PathBuf}, process};

use args::Args;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    
    let source = convert_to_path(&args.source)?;
    let destination = convert_to_path(&args.destination)?;

    let input = match source {
        Some(path) => fs::read_to_string(path)?,
        None => {
            if atty::is(atty::Stream::Stdin) {
                eprintln!("leet -> No input given");
                process::exit(1);
            }

            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let output = program::run(input, args);

    match destination {
        Some(d) => {
            match fs::write(d.clone(), output) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to write to file: {:?}", d);
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        },
        None => {
            match io::stdout().write_all(output.as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to write to stdout");
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
    }

    Ok(())
}

fn convert_to_path(path: &Option<String>) -> Result<Option<PathBuf>> {
    Ok(match path {
        Some(path) => Some(get_path(path)?),
        None => None,
    })
}

fn get_path(path: &str) -> Result<PathBuf>{
    let path = match path {
        p if p.starts_with("~") => {
            dirs::home_dir().ok_or(std::io::Error::from(ErrorKind::NotFound))?.join(&p[2..])
        },
        p if p.starts_with(".") => {
            std::env::current_dir()?.join(&p[2..])
        },
        p => Path::new(&p).to_path_buf(),
    };

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_full_path() {
        let path = "~\\.cargo".to_string();
        let binding = get_path(&path).unwrap();
        let full_path = binding.to_str().unwrap();
        let binding = dirs::home_dir().unwrap().join(".cargo");
        let expected = binding.to_str().unwrap();
        assert_eq!(full_path, expected);
    }

    #[test]
    fn test_get_full_path_absolute() {
        let path = "C:\\Users\\user\\.cargo".to_string();
        let binding = get_path(&path).unwrap();
        let full_path = binding.to_str().unwrap();
        let binding = Path::new("C:\\Users\\user\\.cargo").to_path_buf();
        let expected = binding.to_str().unwrap();
        assert_eq!(full_path, expected);
    }
}