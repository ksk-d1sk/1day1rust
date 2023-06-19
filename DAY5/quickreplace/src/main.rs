use std::env;
use std::fs;

use text_colorizer::*;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let Some(args) = parse_args() else {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            env::args().skip(1).len()
        );
        std::process::exit(1)
    };

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1)
        },
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to replace text: {:?}",
                "Error".red().bold(),
                e
            );
            std::process::exit(1)
        },
    };

    if let Err(e) = fs::write(&args.output, &replaced_data) {
        eprintln!(
            "{} failed to write to file '{}': {:?}",
            "Error".red().bold(),
            args.filename,
            e
        );
        std::process::exit(1);
    }
}

fn replace(
    target: &str,
    replacement: &str,
    text: &str
) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn parse_args() -> Option<Arguments> {
    let mut args = env::args().skip(1);

    Some(Arguments { target      : args.next()?,
                     replacement : args.next()?,
                     filename    : args.next()?,
                     output      : args.next()?, })
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}
