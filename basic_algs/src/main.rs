fn min(array: &[f64; 10]) -> f64 {
    let mut cur_min = array[0];
    for element in array {
        if cur_min > *element {
            cur_min = *element
        }
    }
    return cur_min
}

fn is_prime(n: &i32) -> bool {
    if *n == 1 {
        return false
    }
    if *n == 2 {
        return true
    }
    for i in 2..*n {
        if *n % i == 0 {
            return false
        }
    }
    return true
}

fn nth_prime(n: &i32) -> i32 {
    let mut cur_num = 1;
    let mut i = 0;
    while i != *n {
        cur_num += 1;
        if is_prime(&cur_num) {
            i+=1;
        }
    }
    return cur_num
}

fn bin_search(array: &[f64; 10], key: &f64) -> i32 {
    let mut l: i32 = -1;
    let mut r: i32 = 10;
    while l < r - 1 {
        let m: usize = ((l + r) / 2) as usize;
        if array[m] < *key {
            l = m as i32;
        } else {
            r = m as i32;
        }
    }
    return r
}

fn main() {
    let arr1 = [1.2, 2.0, 4.44, 3.3, 0.67, 1.1, 1.1, 1.1, 1.1, 1.3];
    println!("min element of {:?}: {}", &arr1, min(&arr1));
    let arr2 = [-46.1, 12.01, 111433.434, 323.53, -7.67, -111.11, 2.1, 1.44, 7.1, -111.11];
    println!("min element of {:?}: {}", &arr2, min(&arr2));

    println!("5th prime: {}", nth_prime(&5));
    println!("10th prime: {}", nth_prime(&10));

    let a = [0.000001, 1.9, 2.8, 3.7, 4.6, 5.5, 6.4, 7.3, 8.2, 9.1];
    println!("a: {:?}", &a);
    println!("index of 1.9 in a: {}", bin_search(&a, &1.9));
    println!("index of 4.6 in a: {}", bin_search(&a, &4.6));
    println!("index of 9.1 in a: {}", bin_search(&a, &9.1));
}
