use std::io::Read;

use csv::Reader;

use super::headers;

pub fn print_head(mut reader: Reader<Box<dyn Read>>, head: u32) {
    headers::print_headers(&mut reader);

    let head_iter = reader.records().take(head as usize);
    for rec in head_iter {
        if let Ok(rec) = rec {
            println!(
                "{}",
                rec.iter()
                    .map(|val| {
                        if val.contains(",") {
                            return format!("\"{}\"", val);
                        }

                        return val.to_string();
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
    }
}
