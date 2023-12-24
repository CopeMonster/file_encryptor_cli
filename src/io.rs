use std::fs;

use text_colorizer::Colorize;

pub fn read_file(filename: &String) -> String {
    return match fs::read_to_string(&filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                    "error:".red().bold(), filename, e);
            std::process::exit(1);
        }
    };
}

pub fn write(filename: &String, output: &String, data: &String) {
    match fs::write(&output, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                    "error:".red().bold(), &filename, e);
            std::process::exit(1);
        }
    }
}