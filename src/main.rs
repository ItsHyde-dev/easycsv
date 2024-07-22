use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor, Read};

use clap::ArgAction::Help;
use clap::{Parser, Subcommand};
use csv::ReaderBuilder;
use functions::find::{get_find_tokens, tokenize, validate_token_list};
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::SearcherBuilder;
mod functions;
mod modules;

#[derive(Parser, Debug, Clone)]
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

    #[arg(long, short, long_help = "Select columns from the csv to show.")]
    select: Option<Vec<String>>,

    #[arg(long, short, long_help = "Select columns from the csv to exclude.")]
    exclude: Option<Vec<String>>,

    #[arg(
        long = "dc",
        long_help = "Duplicate count for rows specified. use multiple times to get counts for multiple rows"
    )]
    duplicate_count: Option<Vec<String>>,

    #[arg(short = 'j', long)]
    display_json: bool,

    #[arg(short, long)]
    find: Option<String>,

    #[command(subcommand)]
    commands: Option<Commands>,

    // path to the file
    file_path: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Find {
        #[arg(short, long)]
        columns: Option<Vec<String>>,

        #[arg(long, action = Help)]
        help: Option<bool>,

        /// Number of rows to display
        #[arg(long, short)]
        head: Option<u32>,

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

    let filtered_input: Box<dyn Read>;

    if let Some(query) = args.clone().find {
        let token_list = tokenize(query);

        // sanitize the token list to check if there are any problems with the syntax
        if !validate_token_list(&token_list) {
            panic!("Error in find syntax");
        }

        // build the execution tree
        let terms = get_find_tokens(token_list);

        let filtered_res = search_in_input(input, &terms).unwrap();
        let csv_data = filtered_res.join("\n");

        // Create a Cursor from the String
        filtered_input = Box::new(Cursor::new(csv_data));
    } else {
        filtered_input = input;
    }

    let csv_reader = ReaderBuilder::new().from_reader(filtered_input);

    modules::arg_parser::switch_args(args, csv_reader);
}

fn search_in_input<R: Read>(input: R, terms: &Vec<String>) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    let mut reader = BufReader::new(input);

    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    for term in terms {
        let matcher = RegexMatcher::new(term).unwrap();
        let mut searcher = SearcherBuilder::new().build();

        searcher.search_reader(
            &matcher,
            &mut reader,
            UTF8(|_, line| {
                results.push(line.to_string());
                Ok(true)
            }),
        )?;
    }

    results.insert(0, first_line);

    Ok(results)
}
