use brainfuck_interpreter;
use clap::Parser;
use std::path::Path;

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
    brainfuck_interpreter::interpret(&args.file_path);
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
