use std::collections::{HashMap, HashSet};

use csv::StringRecord;

use super::select;

pub fn print_duplicates(
    i: Box<dyn Iterator<Item = StringRecord>>,
    dup_rows: Vec<String>,
    headers: &Vec<String>,
) -> String {
    let pos_list = select::get_selected_header_position_list(headers, dup_rows.clone());
    let pos_map = get_selected_header_position_map(headers, dup_rows);

    let mut map_set = HashMap::<usize, HashSet<String>>::new();
    let mut duplicates_map = HashMap::<String, HashMap<String, u32>>::new();

    i.for_each(|r| {
        r.iter()
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

    let res: String = duplicates_map
        .iter()
        .map(|(i, x)| {
            x.iter()
                .map(|(j, x)| format!("\n{},{},{}", i, j, x))
                .collect::<String>()
        })
        .collect();

    return "Column,Item,Count".to_string() + &res;
}

pub fn get_selected_header_position_map(
    headers: &Vec<String>,
    select: Vec<String>,
) -> HashMap<usize, String> {
    let mut pos_list: HashMap<usize, String> = HashMap::new();

    for col in select {
        let pos = headers.iter().position(|x| *x == col);

        if let Some(pos) = pos {
            pos_list.insert(pos, col);
        }
    }

    return pos_list;
}
