// Translated implementation of the provided solution

use std::cmp::Ordering;
use std::cmp::min;

pub fn solution() {
    println!(
        "Solution: {:?}",
        exponential_search(
            &vec![-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            &15,
        )
    );
}

fn binary_search<T: Ord>(arr: &Vec<T>, target: &T, mut lo: usize, mut hi: usize) -> Option<usize> {
    while lo < hi {
        let mid = (lo + hi) / 2;

        match arr[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => hi = mid,
            Ordering::Less => lo = mid + 1,
        }
    }

    None
}

fn exponential_search<T: Ord>(arr: &Vec<T>, target: &T) -> Option<usize> {
    if arr.first()? == target {
        return Some(0);
    }

    let n = arr.len();
    let mut i = 1;

    while i < n && arr[i] < *target {
        i *= 2;
    }

    if arr.get(i) == Some(target) {
        return Some(i);
    }

    return binary_search(arr, target, i / 2, min(i, n - 1));
}
