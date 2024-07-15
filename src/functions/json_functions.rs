use std::io::Read;

use csv::{Reader, StringRecord};

// NOTE: This function can be improved in many ways.
// 1. Check the placeholders and throw an error if any of them are not part of the headers
// 2. Allow iterators in the json structure
// 3. Allow the json to be supplied via a file

// DO NOT DELETE CURRENTLY NOT SUPPORTED
#[allow(dead_code)]
pub fn print_json(mut reader: Reader<Box<dyn Read>>, json_structure: String, limit: u32) {
    let mut response: Vec<String> = Vec::new();

    let csv_headers = get_headers_list(&mut reader);
    reader.records().enumerate().for_each(|(index, record)| {
        if limit > 0 && index as u32 >= limit {
            return;
        }

        let mut res_obj = json_structure.clone();

        record
            .unwrap_or(StringRecord::new())
            .iter()
            .enumerate()
            .for_each(|(index, data)| {
                let header = &csv_headers[index];
                let from = "{{header}}".to_string().replace("header", header);
                res_obj = res_obj.replace(&from, data)
            });

        response.push(res_obj);
    });

    println!("{}", format!("[{}]", response.join(",")));
}

pub fn get_headers_list(reader: &mut Reader<Box<dyn Read>>) -> Vec<String> {
    return reader
        .headers()
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
}
