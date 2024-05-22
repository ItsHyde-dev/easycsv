use std::fs::File;

use csv::{Reader, StringRecord};

pub fn print_headers(reader: &mut Reader<File>) {
    let headers = reader
        .headers()
        .unwrap_or(&StringRecord::new())
        .iter()
        .collect::<Vec<&str>>()
        .join(",");

    println!("{}", headers);
}
