use std::fs::File;

use csv::{Reader, StringRecord};

pub fn print_select(path: String, select: Vec<String>, limit: u32) {
    let reader = csv::Reader::from_path(path);
    if let Ok(mut reader) = reader {
        let pos_list = get_selected_header_position_list(&mut reader, select.clone());

        let headers_to_print = reader
            .headers()
            .unwrap_or(&StringRecord::new())
            .iter()
            .filter(|x| select.contains(&x.to_string()))
            .collect::<Vec<&str>>()
            .join(",");

        println!("{}", headers_to_print);

        // get only the indexes that we want and print
        for (i, row) in reader.records().enumerate() {
            if limit != 0 && i as u32 >= limit {
                return;
            }

            if let Ok(row) = row {
                let filtered_row = row
                    .iter()
                    .enumerate()
                    .filter_map(|(col_number, val)| {
                        if pos_list.contains(&col_number) {
                            return Some(val);
                        } else {
                            return None;
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join(",");

                println!("{}", filtered_row);
            }
        }
    }
}

pub fn print_exclude(path: String, exclude: Vec<String>, limit: u32) {
    let reader = csv::Reader::from_path(path);
    if let Ok(mut reader) = reader {
        let pos_list = get_selected_header_position_list(&mut reader, exclude.clone());

        let headers_to_print = reader
            .headers()
            .unwrap_or(&StringRecord::new())
            .iter()
            .filter(|x| !exclude.contains(&x.to_string()))
            .collect::<Vec<&str>>()
            .join(",");

        println!("{}", headers_to_print);

        // get only the indexes that we want and print
        for (i, row) in reader.records().enumerate() {
            if limit != 0 && i as u32 >= limit {
                return;
            }

            if let Ok(row) = row {
                let filtered_row = row
                    .iter()
                    .enumerate()
                    .filter_map(|(col_number, val)| {
                        if !pos_list.contains(&col_number) {
                            return Some(val);
                        } else {
                            return None;
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join(",");

                println!("{}", filtered_row);
            }
        }
    }
}

pub fn get_selected_header_position_list(reader: &mut Reader<File>, select: Vec<String>) -> Vec<usize> {
    let mut pos_list: Vec<usize> = Vec::new();

    for col in select {
        let pos = reader
            .headers()
            .unwrap_or(&StringRecord::new())
            .iter()
            .position(|x| x == col);

        if let Some(pos) = pos {
            pos_list.push(pos);
        }
    }

    return pos_list;
}
