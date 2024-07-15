use crate::functions::find::find;
use std::io::Read;

use csv::{Error, Reader, StringRecord};

use crate::{
    functions::{self},
    Args, Commands,
};

pub fn switch_args(args: Args, mut reader: Reader<Box<dyn Read>>) {
    match args.commands {
        Some(commands) => {
            return switch_subcommands(commands, reader);
        }
        None => {}
    }

    if args.show_headers {
        return functions::headers::print_headers(&mut reader);
    }

    if args.count {
        return functions::count::print_count(reader);
    }

    if let Some(exclude) = args.exclude {
        let limit = args.head.unwrap_or(0);
        return functions::select::print_exclude(reader, exclude, limit);
    }

    if let Some(select) = args.select {
        let limit = args.head.unwrap_or(0);
        return functions::select::print_select(reader, select, limit);
    }

    if let Some(duplicate_count) = args.duplicate_count {
        return functions::duplicate::print_duplicates(reader, duplicate_count);
    }

    if let Some(json_structure) = args.to_json {
        let limit = args.head.unwrap_or(0);
        return functions::json_functions::print_json(reader, json_structure, limit);
    }

    if let Some(head) = args.head {
        return functions::head::print_head(reader, head);
    }
}

fn switch_subcommands(commands: Commands, reader: Reader<Box<dyn Read>>) {
    match commands {
        Commands::Find { columns, query, .. } => find(columns, query, reader),
    }
}

pub fn switch_args_v2(args: Args, mut reader: Reader<Box<dyn Read>>) {
    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|x| x.to_owned())
        .collect();

    let selected = select(
        Box::new(reader.into_records()),
        get_selected(
            args.select.unwrap_or(Vec::new()),
            args.exclude.unwrap_or(Vec::new()),
            &headers,
        ),
    );

    let filtered = filter(selected, args.head);

    // aggregation step
    if args.count {
        println!("Counting...");
        println!("{}", filtered.count());
        return;
    } else if let Some(columns) = args.duplicate_count {
        functions::duplicate::print_duplicates_v2(filtered, columns, &headers);
        return;
    }

    // TODO: will add json support later
    let display_method = DisplayMethod::CSV;

    display(display_method, filtered, &headers);
}

enum DisplayMethod {
    CSV,
    // JSON,
}

fn display(
    display_method: DisplayMethod,
    i: Box<dyn Iterator<Item = StringRecord>>,
    headers: &Vec<String>,
) {
    println!("{}", headers.join(","));
    match display_method {
        DisplayMethod::CSV => i.for_each(|x| {
            println!("{}", x.as_slice());
        }),
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
    // filters
    head: Option<u32>,
) -> Box<dyn Iterator<Item = StringRecord> + 'a> {
    if let Some(head) = head {
        return Box::new(i.take(head as usize));
    }

    return Box::new(i);
}
