use csv::StringRecord;

pub fn print_headers(path: String) {
    let headers = csv::Reader::from_path(path)
        .unwrap()
        .headers()
        .unwrap_or(&StringRecord::new())
        .iter()
        .collect::<Vec<&str>>()
        .join(",");

    println!("{}", headers);
}
