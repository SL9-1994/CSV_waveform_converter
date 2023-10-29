mod csv_parser;
mod data_visualization;
mod integral;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter a CSV file path");

    let mut path: String = String::new();

    io::stdin()
        .read_line(&mut path)
        .map_err(|err| format!("failed to read line: {}", err))?;

    let path: &str = path.trim();

    csv_parser::csv_parse(&path)?;

    Ok(())
}
