use std::io::Read;

use csv::Reader;

use crate::{
    functions::{aggregate, display, filter, select},
    Args,
};

pub fn switch_args(args: Args, mut reader: Reader<Box<dyn Read>>) {
    let mut headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|x| x.to_owned())
        .collect();

    let (selected, headers) =
        select::select(Box::new(reader.into_records()), args.clone(), &mut headers);

    let filtered = filter::filter(selected, args.clone(), &headers);

    match aggregate::aggregate_output_check(&args) {
        true => {
            let agg_val = aggregate::aggregate(filtered, args.clone(), &headers);
            println!("{}", agg_val)
        }
        false => {
            display::display(display::get_display_method(&args), filtered, &headers);
        }
    }
}
