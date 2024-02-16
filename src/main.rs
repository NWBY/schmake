use std::{borrow::Borrow, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    let reader = BufReader::new(File::open(cli.file).expect("cannot open file"));
    
    let mut all_lines: Vec<Vec<String>> = vec![];

    for line in reader.lines() {
        let mut cols: Vec<String> = vec![];
        let line_clone = line.expect("error");
        for word in line_clone.split_whitespace() {
            cols.push(String::from(word));
        }
        
        all_lines.push(cols);
    }
    
    println!("{:?}", all_lines);
    
    let mut output = "";

    for (index, row) in all_lines.iter().enumerate() {
        if index == 0 {
            // first row so this is the table name
            
        }
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}
