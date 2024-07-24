use csv::{Error, StringRecord};

use crate::Args;

pub fn get_selected_header_position_list(headers: &Vec<String>, select: Vec<String>) -> Vec<usize> {
    let mut pos_list: Vec<usize> = Vec::new();

    for col in select {
        let pos = headers.iter().position(|x| *x == col);

        if let Some(pos) = pos {
            pos_list.push(pos);
        }
    }

    return pos_list;
}

pub fn get_selected(
    select: Vec<String>,
    exclude: Vec<String>,
    headers: &Vec<String>,
) -> Vec<usize> {
    let select = match select.len() {
        0 => headers.clone(),
        _ => select,
    };

    return headers
        .iter()
        .enumerate()
        .filter(|(_, x)| select.contains(&x) && !exclude.contains(&x))
        .map(|(i, _)| i)
        .collect();
}

pub fn select<'a>(
    i: Box<dyn Iterator<Item = Result<StringRecord, Error>> + 'a>,
    args: Args,
    headers: &Vec<String>,
) -> (Box<dyn Iterator<Item = StringRecord> + 'a>, Vec<String>) {
    let selected_headers = get_selected(
        args.clone().select.unwrap_or(Vec::new()),
        args.clone().exclude.unwrap_or(Vec::new()),
        &headers,
    );

    let selected_headers_clone = selected_headers.clone();

    let s = i.map(move |rec| {
        rec.unwrap_or(StringRecord::new())
            .iter()
            .enumerate()
            .filter(|(i, _)| selected_headers.contains(&i))
            .map(|(_, x)| x.to_owned())
            .collect::<StringRecord>()
    });

    let h: Vec<String> = headers
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if selected_headers_clone.contains(&i) {
                return Some(x.to_string());
            }

            return None;
        })
        .collect();

    return (Box::new(s), h);
}
