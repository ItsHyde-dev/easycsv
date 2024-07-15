use std::io::Read;

use csv::{Reader, StringRecord};

use super::headers;

pub fn find(columns: Option<Vec<String>>, query: String, mut reader: Reader<Box<dyn Read>>) {
    headers::print_headers(&mut reader);
    let mut allowed_header_positions: Vec<usize> = Vec::new();

    if columns.is_some() {
        allowed_header_positions =
            super::select::get_selected_header_position_list(&mut reader, columns.unwrap())
    }

    reader.records().for_each(|r| {
        let mut should_print: bool = false;
        let record = r.unwrap_or(StringRecord::new()).clone();

        for (idx, val) in record.into_iter().enumerate() {
            if allowed_header_positions.len() > 0 && !allowed_header_positions.contains(&idx) {
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
