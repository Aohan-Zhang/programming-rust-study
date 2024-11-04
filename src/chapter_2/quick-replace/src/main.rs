use std::{env, fs};
use text_colorizer::Colorize;
use crate::model::Arguments;

mod model;

fn main() {
    let args = parse_args();
    let data = read_file(&args.filename);
    let replaced_data = replace(&args.target, &args.replacement, &data)
        .unwrap_or_else(|e| {
            eprintln!("{} - {}", "Error".red().bold(), e);
            std::process::exit(1);
        });
    write_file(&args.output, &replaced_data);
    println!("{} - Wrote {} bytes to {}", "Success".bold().green(), replaced_data.len(), &args.output);
}


fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = regex::Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn write_file(output: &str, content: &str) {
    fs::write(output, &content)
        .unwrap_or_else(|e| {
            eprintln!("{} failed to write to file '{}': {:?}",
                      "Error:".red().bold(), output, e);
            std::process::exit(1);
        })
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("{} failed to read from file '{}': {:?}",
                      "Error:".red().bold(), filename, e);
            std::process::exit(1);
        })
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another",
              "quick-replace".green());
    eprintln!("Usage: quick-replace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}",
                  "Error".red().bold(),
                  args.len());
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}
