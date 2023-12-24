use std::env;
use text_colorizer::Colorize;

#[derive(Debug)]
pub struct Arguments {
    pub encryption: String,
    pub filename: String,
    pub output: String,
}

impl Arguments {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().skip(1).collect();

        if args.len() != 3 {
            print_usage();
            eprintln!("{} wrong number of arguments: expected 3, got {}",
                        "error:".red().bold(), args.len());
                        
            std::process::exit(1); 
        }

        Self {
            encryption: args[0].clone(),
            filename: args[1].clone(),
            output: args[2].clone(),
        }
    }
}

fn print_usage() {
    eprintln!("usage: encrypt <encryption type> <INPUT> <OUTPUT>");
}