use std::{borrow::BorrowMut, fs::File};

use clap::ArgAction::Help;
use clap::Parser;
use csv::{Reader, StringRecord};
mod functions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// Number of rows to display
    #[arg(long, short)]
    head: Option<u32>,

    // Show only headers
    #[arg(long, short = 'H')]
    show_headers: bool,

    #[arg(long, action = Help)]
    help: Option<bool>,

    #[arg(long, short)]
    select: Option<Vec<String>>,

    // path to the file
    file_path: String,
}

fn main() {
    let args = Args::parse();
    switch_args(args)
}

fn switch_args(args: Args) {
    if args.show_headers {
        return print_headers(
            csv::Reader::from_path(&args.file_path)
                .unwrap()
                .borrow_mut(),
        );
    }

    if let Some(select) = args.select {
        let limit = args.head.unwrap_or(0);
        return print_select(args.file_path, select, limit);
    }

    if let Some(head) = args.head {
        return print_head(args.file_path, head);
    }
}

fn print_select(path: String, select: Vec<String>, limit: u32) {
    let reader = csv::Reader::from_path(path);
    if let Ok(mut reader) = reader {
        let pos_list = get_selected_header_position_list(&mut reader, select.clone());

        let headers_to_print = reader
            .headers()
            .unwrap_or(&StringRecord::new())
            .iter()
            .filter(|x| select.contains(&x.to_string()))
            .collect::<Vec<&str>>()
            .join(",");

        println!("{}", headers_to_print);

        // get only the indexes that we want and print
        for (i, row) in reader.records().enumerate() {
            if limit != 0 && i as u32 >= limit {
                return;
            }

            if let Ok(row) = row {
                let filtered_row = row
                    .iter()
                    .enumerate()
                    .filter_map(|(col_number, val)| {
                        if pos_list.contains(&col_number) {
                            return Some(val);
                        } else {
                            return None;
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join(",");

                println!("{}", filtered_row);
            }
        }
    }
}

fn get_selected_header_position_list(reader: &mut Reader<File>, select: Vec<String>) -> Vec<usize> {
    let mut pos_list: Vec<usize> = Vec::new();

    for col in select {
        let pos = reader
            .headers()
            .unwrap_or(&StringRecord::new())
            .iter()
            .position(|x| x == col);

        if let Some(pos) = pos {
            pos_list.push(pos);
        }
    }

    return pos_list;
}

fn print_head(path: String, head: u32) {
    let reader = csv::Reader::from_path(path);
    if let Ok(mut reader) = reader {
        print_headers(reader.borrow_mut());

        let head_iter = reader.records().take(head as usize);
        for rec in head_iter {
            if let Ok(rec) = rec {
                println!("{}", rec.iter().collect::<Vec<&str>>().join(","))
            }
        }
    }
}

fn print_headers(reader: &mut Reader<File>) {
    let headers = reader
        .headers()
        .unwrap_or(&StringRecord::new())
        .iter()
        .collect::<Vec<&str>>()
        .join(",");

    println!("{}", headers);
}
