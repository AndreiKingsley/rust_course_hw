use untitled1::scoring_table::{OldFormatTable, transform, score};
use untitled1::sublist::compare;

fn main() {
    let a = [Some(3), None, Some(2), None];
    let b = [None, Some(2), None];
    println!("{:?}", &a[1..4]);
    println!("{:?}", &b);
    println!("{:?}", compare(&a[1..4], &b));
}
