use std::io::Read;

use csv::Reader;

pub fn print_count(mut reader: Reader<Box<dyn Read>>) {
    println!("{}", reader.records().count());
}
