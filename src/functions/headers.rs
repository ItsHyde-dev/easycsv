use std::io::Read;

use csv::{Reader, StringRecord};

pub fn print_headers(reader: &mut Reader<Box<dyn Read>>) {
    let headers = reader
        .headers()
        .unwrap_or(&StringRecord::new())
        .iter()
        .collect::<Vec<&str>>()
        .join(",");

    println!("{}", headers);
}
