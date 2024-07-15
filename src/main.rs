use std::fs::File;
use std::io::{self, Read};

use clap::ArgAction::Help;
use clap::{Parser, Subcommand};
use csv::ReaderBuilder;
mod functions;
mod modules;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// Number of rows to display
    #[arg(long, short)]
    head: Option<u32>,

    /// Number of rows in csv
    #[arg(long, short)]
    count: bool,

    // Show only headers
    #[arg(long, short = 'H', long_help = "Show the headers of the csv file")]
    show_headers: bool,

    #[arg(long, action = Help)]
    help: Option<bool>,

    #[arg(
        long,
        short,
        long_help = "Select columns from the csv to show.\nCannot use with exclude.",
        conflicts_with = "exclude"
    )]
    select: Option<Vec<String>>,

    #[arg(
        long,
        short,
        long_help = "Select columns from the csv to exclude.\nCannot use with select.",
        conflicts_with = "select"
    )]
    exclude: Option<Vec<String>>,

    #[arg(
        long = "dc",
        long_help = "Duplicate count for rows specified. use multiple times to get counts for multiple rows"
    )]
    duplicate_count: Option<Vec<String>>,

    #[arg(short = 'j', long)]
    to_json: Option<String>,

    #[command(subcommand)]
    commands: Option<Commands>,

    // path to the file
    file_path: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Find {
        #[arg(short, long)]
        columns: Option<Vec<String>>,

        #[arg(long, action = Help)]
        help: Option<bool>,

        query: String,
    },
}

fn main() {
    let args = Args::parse();

    // ignore the sigpipe error
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let input: Box<dyn Read> = if atty::isnt(atty::Stream::Stdin) {
        Box::new(io::stdin())
    } else if let Some(ref file_path) = args.file_path {
        Box::new(File::open(file_path).expect("Unable to open file"))
    } else {
        eprintln!("No input provided. Use either a file path or pipe data to the program.");
        std::process::exit(1);
    };

    let csv_reader = ReaderBuilder::new().from_reader(input);

    // modules::arg_parser::switch_args(args, csv_reader)
    modules::arg_parser::switch_args_v2(args, csv_reader)
}
