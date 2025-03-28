use std::cmp::Ordering;

fn main() {
    println!("{:?}", binary_search(&[-1, 1, 2, 3, 5, 6, 7, 8, 9, 10], &0));
}

fn binary_search<T: Ord>(nums: &[T], n: &T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = nums.len();

    while lo < hi {
        let i = lo + (hi - lo) / 2; // lo + (hi - lo) to avoid overflow

        match nums[i].cmp(n) {
            Ordering::Equal => return Some(i),
            Ordering::Greater => hi = i,
            Ordering::Less => lo = i + 1,
        }
    }

    None
}

//   h
//                                 l
//                  i
// [-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
