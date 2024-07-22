use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use clap::ArgAction::Help;
use clap::{Parser, Subcommand};
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

    let mut input: Box<dyn Read> = if atty::isnt(atty::Stream::Stdin) {
        Box::new(io::stdin())
    } else if let Some(ref file_path) = args.file_path {
        Box::new(File::open(file_path).expect("Unable to open file"))
    } else {
        eprintln!("No input provided. Use either a file path or pipe data to the program.");
        std::process::exit(1);
    };

    let filtered_res: Vec<String>;
    let headers: String;

    if let Some(query) = args.find {
        let token_list = tokenize(query);

        // sanitize the token list to check if there are any problems with the syntax
        if !validate_token_list(&token_list) {
            panic!("Error in find syntax");
        }

        // build the execution tree
        let terms = get_find_tokens(token_list);

        (headers, filtered_res) = search_in_input(input, &terms).unwrap();
    } else {
        let mut buf = String::new();
        let _ = input.read_to_string(&mut buf);
        let lines: Vec<String> = buf.split("\n").map(|x| x.to_string()).collect();
        if lines.len() == 0 {
            return;
        }

        headers = lines[0].clone();
        filtered_res = lines.iter().skip(1).map(|x| x.to_owned()).collect();
    }

    dbg!(headers, filtered_res);

    // let csv_reader = ReaderBuilder::new().from_reader(input);

    modules::arg_parser::switch_args(args, headers, filtered_res);
}

fn search_in_input<R: Read>(input: R, terms: &Vec<String>) -> io::Result<(String, Vec<String>)> {
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

    Ok((first_line, results))
}
