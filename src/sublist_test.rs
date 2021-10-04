#[cfg(test)]
mod tests {
    use crate::sublist::compare;
    use crate::sublist::Comparison::*;

    #[test]
    fn test_compare_int() {
        let a = [1, 2, 3, 4, 5, 6];
        let b = [1, 2, 3, 4];
        assert_eq!(compare(&a, &b), Superlist);
        assert_eq!(compare(&b, &a), Sublist);
        assert_eq!(compare(&a[0..4], &b), Equal);
        assert_eq!(compare(&b, &a[1..4]), Superlist);
        assert_eq!(compare(&a[1..5], &b), Other);
        assert_eq!(compare(&b[1..2], &a[5..6]), Other);
    }

    #[test]
    fn test_compare_option() {
        let a = [Some(3), None, Some(2), None];
        let b = [None, Some(2), None];
        assert_eq!(compare(&a, &b), Superlist);
        assert_eq!(compare(&b, &a), Sublist);
        assert_eq!(compare(&a[1..4], &b), Equal);
        assert_eq!(compare(&b[1..], &a[0..2]), Other);
    }
}
