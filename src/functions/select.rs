use std::io::Read;

use csv::{Reader, StringRecord};

pub fn print_select(mut reader: Reader<Box<dyn Read>>, select: Vec<String>, limit: u32) {
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
                        if val.contains(",") {
                            return Some(format!("\"{}\"", val));
                        }
                        return Some(val.to_string());
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<String>>()
                .join(",");

            println!("{}", filtered_row);
        } else {
            println!("Error reading records");
        }
    }
}

pub fn print_exclude(mut reader: Reader<Box<dyn Read>>, exclude: Vec<String>, limit: u32) {
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
                        if val.contains(",") {
                            return Some(format!("\"{}\"", val));
                        }
                        return Some(val.to_string());
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<String>>()
                .join(",");

            println!("{}", filtered_row);
        }
    }
}

pub fn get_selected_header_position_list(
    reader: &mut Reader<Box<dyn Read>>,
    select: Vec<String>,
) -> Vec<usize> {
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

pub fn get_selected_header_position_list_v2(
    headers: &Vec<String>,
    select: Vec<String>,
) -> Vec<usize> {
    let mut pos_list: Vec<usize> = Vec::new();

    for col in select {
        let pos = headers.iter().position(|x| *x == col);

        if let Some(pos) = pos {
            pos_list.push(pos);
        }
    }

    return pos_list;
}
