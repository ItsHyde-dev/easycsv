use csv::StringRecord;

use crate::Args;

use super::duplicate;

pub fn aggregate(
    i: Box<dyn Iterator<Item = StringRecord>>,
    args: Args,
    headers: &Vec<String>,
) -> String {
    if let Some(columns) = args.duplicate_count {
        return duplicate::print_duplicates(i, columns, &headers);
    } else if args.count {
        println!("Counting...");
        return i.count().to_string();
    }

    return String::new();
}

pub fn aggregate_output_check(args: &Args) -> bool {
    return args.count || args.duplicate_count.is_some();
}
