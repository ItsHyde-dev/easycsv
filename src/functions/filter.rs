use csv::StringRecord;

use crate::Args;

pub fn filter(
    i: Box<dyn Iterator<Item = StringRecord>>,
    args: Args,
    headers: &Vec<String>,
) -> Box<dyn Iterator<Item = StringRecord>> {
    let mut i = i;

    if let Some(query) = args.find {
        i = Box::new(super::find::find(i, query, headers.clone()))
    }

    if let Some(head) = args.head {
        i = Box::new(i.take(head as usize));
    }

    return Box::new(i);
}
