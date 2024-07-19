use csv::StringRecord;

use crate::{Args, Commands};

pub fn filter(
    i: Box<dyn Iterator<Item = StringRecord>>,
    args: Args,
    headers: &Vec<String>,
) -> Box<dyn Iterator<Item = StringRecord>> {
    let mut i = i;

    if let Some(query) = args.find {
        i = Box::new(super::find::find(i, query))
    }

    // TODO: Change this to be handled via flags instead of subcommands
    if let Some(command) = args.commands {
        match command {
            Commands::Find {
                columns,
                query,
                head,
                ..
            } => {
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
