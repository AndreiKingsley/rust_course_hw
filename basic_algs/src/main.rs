use std::fs::OpenOptions;

fn min(array: &[f64; 10]) -> f64 {
    let mut cur_min = array[0];
    for element in array {
        if cur_min > *element {
            cur_min = *element
        }
    }
    cur_min
}

fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn nth_prime(n: i32) -> i32 {
    let mut cur_num = 1;
    let mut i = 0;
    while i != n {
        cur_num += 1;
        if is_prime(cur_num) {
            i += 1;
        }
    }
    cur_num
}

fn bin_search(array: &[f64; 10], key: f64) -> Option<usize> {
    let mut l: usize = 0;
    let mut r: usize = 10;
    while l < r - 1 {
        let m: usize = ((l + r) / 2);
        if array[m] < key {
            l = m;
        } else {
            r = m;
        }
    }
    if r == 10 || array[r as usize] != key {
        return None;
    }
    Option::from(r)
}

fn print_bin_search_result(array: &[f64; 10], key: f64) {
    match bin_search(array, key) {
        None => println!("{} isn't in a", key),
        Some(ind) => println!("index of {} in a: {}", key, ind),
    }
}

fn main() {
    let arr1 = [1.2, 2.0, 4.44, 3.3, 0.67, 1.1, 1.1, 1.1, 1.1, 1.3];
    println!("min element of {:?}: {}", &arr1, min(&arr1));
    let arr2 = [-46.1, 12.01, 111433.434, 323.53, -7.67, -111.11, 2.1, 1.44, 7.1, -111.11];
    println!("min element of {:?}: {}", &arr2, min(&arr2));

    println!("5th prime: {}", nth_prime(5));
    println!("10th prime: {}", nth_prime(10));

    let a = [0.000001, 1.9, 2.8, 3.7, 4.6, 5.5, 6.4, 7.3, 8.2, 9.1];
    println!("a: {:?}", &a);
    print_bin_search_result(&a, 1.9);
    print_bin_search_result(&a, 4.6);
    print_bin_search_result(&a, 10.12);
    print_bin_search_result(&a, 9.1);
    print_bin_search_result(&a, 0.0);
}
