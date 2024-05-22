use clap::Parser;
mod functions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of rows to display
    #[arg(short, long)]
    head: u32,
}

fn main() {
    let args = Args::parse();
    // create hierarchy of flags
}

use std::{error::Error, io, process};

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
