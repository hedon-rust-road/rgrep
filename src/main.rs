use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use colored::Colorize;
use glob::glob;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(anyhow::anyhow!("Usage: rgrep <pattern> <directory>"));
    }
    let pattern: String = args[1].clone();
    let path = args[2].clone();

    handle_path(&path, &pattern)?;
    Ok(())
}

fn handle_path(path: &str, pattern: &str) -> anyhow::Result<()> {
    for entry in (glob(path)?).flatten() {
        if entry.is_file() {
            grep_file(pattern, &entry.to_string_lossy())?;
        } else if entry.is_dir() {
            handle_dir(&entry.to_string_lossy(), pattern)?;
        }
    }
    Ok(())
}

fn handle_dir(dir: &str, pattern: &str) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            grep_file(pattern, path.to_string_lossy().as_ref())?;
        } else if path.is_dir() {
            handle_dir(path.to_string_lossy().as_ref(), pattern)?;
        }
    }
    Ok(())
}

fn grep_file(pattern: &str, filename: &str) -> anyhow::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut printed = false;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let re = Regex::new(pattern)?;
        let founds = re.find_iter(&line);
        for found in founds {
            if !printed {
                println!("{}", filename);
                printed = true;
            }
            let start = found.start();
            let end = found.end();
            let mut line = line.clone();
            let colored_pattern = &line[start..end].red().to_string();
            line.replace_range(start..end, colored_pattern);
            println!("    {}:{} {}", i + 1, start, line.trim());
        }
    }
    Ok(())
}
