use std::collections::HashMap;

pub type OldFormatTable = HashMap<i32, Vec<char>>;
pub type NewFormatTable = HashMap<char, i32>;

pub fn transform(old_table: &OldFormatTable) -> NewFormatTable {
    old_table.iter().flat_map(
        |(k, val)| {
            val.iter().map(|x| { (x.to_ascii_lowercase(), *k) }).collect::<Vec<_>>()
        }
    ).collect()
}

pub fn score(input: &str, table: &NewFormatTable) -> i32 {
    input.to_ascii_lowercase().chars().fold(0, |acc, x|
        {
            match table.get(&x) {
                None => acc,
                Some(val) => acc + *val
            }
        },
    )
}