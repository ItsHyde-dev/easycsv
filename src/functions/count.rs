pub fn print_count(path: String) {
    println!(
        "{}",
        csv::Reader::from_path(path).unwrap().records().count()
    );
}
