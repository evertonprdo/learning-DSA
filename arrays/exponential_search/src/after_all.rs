// An idea I had after all

use std::cmp::Ordering;
use std::cmp::min;

pub fn after_all() {
    println!(
        "After All: {:?}",
        exponential_search(
            &vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32
            ],
            &27,
        )
    );
}

// This isn't better than the pointer-based approach because it is a recursive implementation that is at risk of stack overflow.
// But I enjoyed trying
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let i = arr.len() / 2;

    match arr[i].cmp(target) {
        Ordering::Equal => Some(i),
        Ordering::Greater => binary_search(&arr[..i], target),
        Ordering::Less => {
            let lo = i + 1;
            binary_search(&arr[lo..], target).map(|i| i + lo)
        }
    }
}

fn exponential_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
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

    let lo = i / 2;
    return binary_search(&arr[lo..min(i, n - 1)], target).map(|i| i + lo);
}
