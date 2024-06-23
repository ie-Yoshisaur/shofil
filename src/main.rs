use glob::glob;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
enum FileReaderError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Glob pattern error: {0}")]
    Glob(#[from] glob::PatternError),
    #[error("Glob matching error: {0}")]
    GlobMatch(#[from] glob::GlobError),
    #[error("No glob patterns provided")]
    NoPatterns,
}

type Result<T> = std::result::Result<T, FileReaderError>;

fn main() -> Result<()> {
    let patterns = get_patterns_from_args()?;
    process_patterns(&patterns)
}

fn get_patterns_from_args() -> Result<Vec<String>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <glob_pattern1> [glob_pattern2] ...", args[0]);
        return Err(FileReaderError::NoPatterns);
    }
    Ok(args[1..].to_vec())
}

fn process_patterns(patterns: &[String]) -> Result<()> {
    patterns
        .par_iter()
        .try_for_each(|pattern| process_pattern(pattern))
}

fn process_pattern(pattern: &str) -> Result<()> {
    let paths: Vec<_> = glob(pattern)?.collect::<std::result::Result<_, _>>()?;

    paths.par_iter().try_for_each(|path| process_path(path))
}

fn process_path(path: &Path) -> Result<()> {
    if let Some(filename) = path.file_name() {
        let filename_str = filename.to_string_lossy();
        let output = Mutex::new(String::new());

        output
            .lock()
            .unwrap()
            .push_str(&format!("---{}---\n", filename_str));

        if path.is_file() {
            print_file_contents(path, &output)?;
        } else {
            output.lock().unwrap().push_str("Not a file\n");
        }

        output.lock().unwrap().push('\n');

        print!("{}", output.lock().unwrap());
    }
    Ok(())
}

fn print_file_contents(path: &Path, output: &Mutex<String>) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        output.lock().unwrap().push_str(&format!("{}\n", line?));
    }
    Ok(())
}
