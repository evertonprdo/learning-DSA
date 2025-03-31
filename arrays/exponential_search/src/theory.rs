use std::cmp::Ordering;

// This solution is based on theory
pub fn theory() {
    println!(
        "Theory: {:?}",
        exponential_search(
            &vec![-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            &14,
        )
    );
}

fn binary_search<T: Ord>(ipt: &Vec<T>, target: &T, mut l: usize, mut r: usize) -> Option<usize> {
    while l < r {
        let i = l + (r - l) / 2;
        match ipt[i].cmp(target) {
            Ordering::Equal => return Some(i),
            Ordering::Greater => r = i,
            Ordering::Less => l = i + 1,
        }
    }
    None
}

fn exponential_search<T: Ord>(ipt: &Vec<T>, target: &T) -> Option<usize> {
    if ipt.first()? == target {
        return Some(0);
    }

    let mut r = 1;
    let mut l = 0;

    while r < ipt.len() {
        match ipt[r].cmp(target) {
            Ordering::Equal => return Some(r),
            Ordering::Greater => return binary_search(ipt, target, l, r),

            _ => {}
        }

        l = r;
        r *= 2;
    }

    None
}

//   l
//      r
//   i
// [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
// [-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
