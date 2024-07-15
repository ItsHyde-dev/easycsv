use std::{collections::HashMap, io::Read};

use csv::{Error, Reader, StringRecord};
use serde_json::json;

use crate::{
    functions::{self},
    Args, Commands,
};

pub fn switch_args(args: Args, mut reader: Reader<Box<dyn Read>>) {
    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|x| x.to_owned())
        .collect();

    let selected = select(
        Box::new(reader.into_records()),
        get_selected(
            args.clone().select.unwrap_or(Vec::new()),
            args.clone().exclude.unwrap_or(Vec::new()),
            &headers,
        ),
    );

    let filtered = filter(selected, args.clone(), &headers);

    // aggregation step
    if args.count {
        println!("Counting...");
        println!("{}", filtered.count());
        return;
    } else if let Some(columns) = args.duplicate_count {
        functions::duplicate::print_duplicates(filtered, columns, &headers);
        return;
    }

    let mut display_method = DisplayMethod::CSV;
    if args.show_headers {
        display_method = DisplayMethod::OnlyHeaders;
    } else if args.display_json {
        display_method = DisplayMethod::JSON;
    }

    display(display_method, filtered, &headers);
}

enum DisplayMethod {
    OnlyHeaders,
    CSV,
    JSON,
}

fn display(
    display_method: DisplayMethod,
    i: Box<dyn Iterator<Item = StringRecord>>,
    headers: &Vec<String>,
) {
    match display_method {
        DisplayMethod::CSV => {
            println!("{}", headers.join(","));
            i.for_each(|x| {
                println!("{}", x.as_slice());
            });
        }
        DisplayMethod::OnlyHeaders => {
            println!("{}", headers.join(","));
        }
        DisplayMethod::JSON => {
            i.for_each(|x| {
                let mut print_map: HashMap<String, String> = HashMap::new();
                x.iter().zip(headers.iter()).for_each(|(val, header)| {
                    print_map.insert(header.to_owned(), val.to_string());
                });

                println!("{}", json!(print_map));
            });
        }
    }
}

fn get_selected(select: Vec<String>, exclude: Vec<String>, headers: &Vec<String>) -> Vec<usize> {
    let select = match select.len() {
        0 => headers.clone(),
        _ => select,
    };

    return headers
        .iter()
        .enumerate()
        .filter(|(_, x)| select.contains(&x) && !exclude.contains(&x))
        .map(|(i, _)| i)
        .collect();
}

fn select<'a>(
    i: Box<dyn Iterator<Item = Result<StringRecord, Error>> + 'a>,
    selected_headers: Vec<usize>,
) -> Box<dyn Iterator<Item = StringRecord> + 'a> {
    let s = i.map(move |rec| {
        rec.unwrap_or(StringRecord::new())
            .iter()
            .enumerate()
            .filter(|(i, _)| selected_headers.contains(&i))
            .map(|(_, x)| x.to_owned())
            .collect::<StringRecord>()
    });

    return Box::new(s);
}

fn filter<'a>(
    i: Box<dyn Iterator<Item = StringRecord> + 'a>,
    args: Args,
    headers: &Vec<String>,
) -> Box<dyn Iterator<Item = StringRecord> + 'a> {
    let mut i = i;
    if let Some(command) = args.commands {
        match command {
            Commands::Find {
                columns,
                query,
                head,
                ..
            } => {
                // code to find
                // get column indices
                let indices: Vec<usize> = headers
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if let Some(columns) = &columns {
                            if columns.contains(&x) {
                                return Some(i);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    })
                    .collect();

                i = Box::new(i.filter(move |x| {
                    x.iter()
                        .enumerate()
                        .filter(|(i, y)| {
                            let cont = y.to_string().contains(&query);
                            if indices.len() == 0 {
                                return cont;
                            }

                            return cont && indices.contains(&i);
                        })
                        .count()
                        > 0
                }));

                if let Some(head) = head {
                    i = Box::new(i.take(head as usize));
                }
            }
        };
    }

    if let Some(head) = args.head {
        i = Box::new(i.take(head as usize));
    }

    return Box::new(i);
}
