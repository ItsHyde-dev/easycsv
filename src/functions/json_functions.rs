use csv::StringRecord;

pub fn print_json(path: String, json_structure: String, limit: u32) {
    let json_sample: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&json_structure);

    if json_sample.is_err() {
        println!("Json template is invalid");
        return;
    }

    let mut response: Vec<String> = Vec::new();

    let mut reader = csv::Reader::from_path(path.clone()).unwrap();
    let csv_headers = get_headers_list(path);
    reader.records().enumerate().for_each(|(index, record)| {
        if limit > 0 && index as u32 >= limit {
            return;
        }

        record
            .unwrap_or(StringRecord::new())
            .iter()
            .enumerate()
            .for_each(|(index, data)| {
                let header = &csv_headers[index];
                let from = format!("{{{header}}}");
                response.push(json_structure.replace(&from, data));
            });
    });

    println!("{}", format!("[{}]", response.join(",")));
}

pub fn get_headers_list(path: String) -> Vec<String> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    return reader
        .headers()
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
}
