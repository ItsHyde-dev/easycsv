use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use csv::{Reader, StringRecord};

use super::select::get_selected_header_position_list;
pub fn print_duplicates(path: String, dup_rows: Vec<String>) {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let pos_list = get_selected_header_position_list(&mut reader, dup_rows.clone());
    let pos_map = get_selected_header_position_map(&mut reader, dup_rows);

    let mut map_set = HashMap::<usize, HashSet<String>>::new();
    let mut duplicates_map = HashMap::<String, HashMap<String, u32>>::new();

    reader.records().for_each(|r| {
        r.unwrap()
            .iter()
            .enumerate()
            .filter(|(index, _)| pos_list.contains(index))
            .for_each(|(index, val)| {
                let set = map_set.get_mut(&index);
                match set {
                    Some(dup_set) => match dup_set.contains(val) {
                        true => {
                            match duplicates_map.get_mut(pos_map.get(&index).unwrap()) {
                                Some(dup_map) => match dup_map.get(val) {
                                    Some(count) => {
                                        dup_map.insert(val.to_string(), count + 1);
                                    }
                                    None => {
                                        dup_map.insert(val.to_string(), 2);
                                    }
                                },
                                None => {
                                    let mut new_map = HashMap::new();
                                    new_map.insert(val.to_string(), 2);
                                    duplicates_map
                                        .insert(pos_map.get(&index).unwrap().to_string(), new_map);
                                }
                            };
                        }
                        false => {
                            dup_set.insert(val.to_string());
                        }
                    },
                    None => {
                        let mut new_set = HashSet::new();
                        new_set.insert(val.to_string());
                        map_set.insert(index, new_set);
                    }
                }
            });
    });

    duplicates_map.iter().for_each(|(i, x)| {
        x.iter()
            .for_each(|(j, x)| println!("Column: {}, Entry: {}, Count: {}", i, j, x))
    });
}

pub fn get_selected_header_position_map(
    reader: &mut Reader<File>,
    select: Vec<String>,
) -> HashMap<usize, String> {
    let mut pos_list: HashMap<usize, String> = HashMap::new();

    for col in select {
        let pos = reader
            .headers()
            .unwrap_or(&StringRecord::new())
            .iter()
            .position(|x| x == col);

        if let Some(pos) = pos {
            pos_list.insert(pos, col);
        }
    }

    return pos_list;
}
