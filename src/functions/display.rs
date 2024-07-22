use std::collections::HashMap;

use csv::StringRecord;
use serde_json::json;

use crate::Args;

pub enum DisplayMethod {
    OnlyHeaders,
    CSV,
    JSON,
}

pub fn get_display_method(args: &Args) -> DisplayMethod {
    if args.show_headers {
        return DisplayMethod::OnlyHeaders;
    } else if args.display_json {
        return DisplayMethod::JSON;
    } else {
        return DisplayMethod::CSV;
    }
}

pub fn display(
    display_method: DisplayMethod,
    i: Box<dyn Iterator<Item = StringRecord>>,
    headers: &Vec<String>,
) {
    println!("reached here");
    match display_method {
        DisplayMethod::CSV => {
            println!("{}", headers.join(","));
            i.for_each(|x| {
                println!("{}", x.iter().collect::<Vec<&str>>().join(","));
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
