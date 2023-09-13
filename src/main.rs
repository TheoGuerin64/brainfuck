pub mod interpreter;
pub mod parser;

use clap::Parser;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Source file path
    #[arg(value_parser = source_file_extension)]
    file_path: String,
}

fn source_file_extension(s: &str) -> Result<String, String> {
    let path: &Path = Path::new(s);
    if !path.extension().is_some_and(|s| s == "bf") {
        return Err(String::from("source file extension is not valid"));
    };

    Ok(s.to_string())
}

fn main() {
    let args: Args = Args::parse();

    let content = fs::read_to_string(args.file_path).unwrap_or_else(|error| {
        eprintln!("file reading error: {error}");
        process::exit(1);
    });
    let instructions = parser::parse(&content).unwrap_or_else(|error| {
        eprintln!("parsing error: {error}");
        process::exit(1);
    });
    if let Err(error) = interpreter::execute(&instructions) {
        eprintln!("execution error: {error}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_file_extension_valid() {
        let result = source_file_extension("test.bf");
        assert!(
            result.is_ok(),
            "source file extension failed, value was {:?}",
            result
        );
    }

    #[test]
    fn source_file_extension_invalid() {
        let result = source_file_extension("test.txt");
        assert!(
            result.is_err(),
            "source file extension failed, value was {:?}",
            result
        );
    }
}
