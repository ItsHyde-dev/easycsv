use super::headers;

pub fn print_head(path: String, head: u32) {
    let reader = csv::Reader::from_path(path.clone());
    if let Ok(mut reader) = reader {
        headers::print_headers(path);

        let head_iter = reader.records().take(head as usize);
        for rec in head_iter {
            if let Ok(rec) = rec {
                println!(
                    "{}",
                    rec.iter()
                        .map(|val| {
                            if val.contains(",") {
                                return format!("\"{}\"", val);
                            }

                            return val.to_string();
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}
