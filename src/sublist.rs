#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comparison {
    Equal,
    // список `a` равен списку `b`
    Sublist,
    // список `a` является подсписком `b`
    Superlist,
    // список `b` является подсписком `a`
    Other, // списки не равны и не являются подсписками друг друга
}

fn check_equal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).fold(
        true,
        |acc, (x, y)| {
            acc && (*x == *y)
        },
    )
}

pub fn compare<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    let (a, b, swapped) = if a.len() < b.len() {
        (b, a, true)
    } else {
        (a, b, false)
    };
    let mut status = false;
    for i in 0..(a.len() - b.len() + 1) {
        if check_equal(&a[i..i + b.len()], b) {
            status = true;
            break;
        }
    }
    if status {
        if a.len() == b.len() {
            Comparison::Equal
        } else if swapped {
            Comparison::Sublist
        } else {
            Comparison::Superlist
        }
    } else {
        Comparison::Other
    }
}
