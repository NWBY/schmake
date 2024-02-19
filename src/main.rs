use std::{
    borrow::Borrow,
    fmt::format,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

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

    // let mut output = String::from("");
    let f = File::create("./test.sql").expect("unable to create file");
    let mut f = BufWriter::new(f);

    for (index, row) in all_lines.iter().enumerate() {
        // for (inner_index, token) in row.iter().enumerate() {
        if row[0].as_str() == "table" {
            let output = format!("CREATE TABLE [IF NOT EXISTS] {} (", row[index + 1]);
            write!(f, "{}", output).expect("unable to write");
            // fs::write("./test.sql", output).expect("Unable to write file");
        } else {
            let mut clone = row.clone();
            let col = &row[0];
            let col_type = &row[1];
            clone.remove(0);
            clone.remove(0);
            let addition = format!("\n\t{} {} {:?}", col, calculate_col_type(col_type), clone);
            write!(f, "{}", addition).expect("unable to write");
        }

        if index == all_lines.len() - 1 {
            write!(f, "\n);").expect("unable to write");
        }

        // output.push_str(&addition);
        // }
    }

    // print!("{:?}", output);

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

fn calculate_col_type(col_type: &String) -> &str {
    let datatype = match col_type.as_str() {
        // Integer types
        "serial" | "SERIAL" => "SERIAL",
        "int" | "INT" => "INT",
        "smallint" | "SMALLINT" => "SMALLINT",

        // Character types
        "char" | "CHAR" => "CHAR",
        "varchar" | "VARCHAR" => "VARCHAR",
        "text" | "TEXT" => "TEXT",
        _ => "Hello World",
    };

    return datatype;
}
