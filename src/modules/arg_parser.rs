use std::io::Read;

use csv::{Reader, StringRecord};

use crate::{functions, Args, Commands};

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

fn switch_subcommands(commands: Commands, mut reader: Reader<Box<dyn Read>>) {
    match commands {
        Commands::Find { columns, query, .. } => {
            functions::headers::print_headers(&mut reader);
            let mut allowed_header_positions: Vec<usize> = Vec::new();

            if columns.is_some() {
                allowed_header_positions = functions::select::get_selected_header_position_list(
                    &mut reader,
                    columns.unwrap(),
                )
            }

            reader.records().for_each(|r| {
                let mut should_print: bool = false;
                let record = r.unwrap_or(StringRecord::new()).clone();

                for (idx, val) in record.into_iter().enumerate() {
                    if allowed_header_positions.len() > 0
                        && !allowed_header_positions.contains(&idx)
                    {
                        continue;
                    } else {
                        if val.contains(&query) {
                            should_print = true;
                            break;
                        }
                    }
                }

                if should_print {
                    println!("{}", record.iter().collect::<Vec<&str>>().join(","));
                }
            })
        }
    }
}
