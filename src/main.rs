use std::borrow::BorrowMut;

use clap::ArgAction::Help;
use clap::Parser;
mod functions;

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
    #[arg(long, short = 'H')]
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

    // path to the file
    file_path: String,
}

fn main() {
    let args = Args::parse();
    switch_args(args)
}

fn switch_args(args: Args) {
    if args.show_headers {
        return functions::headers::print_headers(
            csv::Reader::from_path(&args.file_path)
                .unwrap()
                .borrow_mut(),
        );
    }

    if args.count {
        return functions::count::print_count(args.file_path);
    }

    if let Some(exclude) = args.exclude {
        let limit = args.head.unwrap_or(0);
        return functions::select::print_exclude(args.file_path, exclude, limit);
    }

    if let Some(select) = args.select {
        let limit = args.head.unwrap_or(0);
        return functions::select::print_select(args.file_path, select, limit);
    }

    if let Some(head) = args.head {
        return functions::head::print_head(args.file_path, head);
    }
}
