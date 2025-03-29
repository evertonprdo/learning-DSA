// 3090. Maximum Length Substring With Two Occurrences: https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/
use std::collections::HashMap;

fn main() {
    println!("{}", maximum_length_substring("bcbbbcba".to_string()));
    println!("{}", maximum_length_substring_res("bcbbbcba".to_string()));
}

// My implementation based on theory
fn maximum_length_substring(s: String) -> i32 {
    let mut map: HashMap<&u8, i32> = HashMap::new();
    let bytes = s.as_bytes();

    let mut res = 0;
    let mut win = 0;
    let mut j = 0;

    for byte in bytes.iter() {
        map.entry(byte).and_modify(|x| *x += 1).or_insert(1);
        win += 1;

        while *map.get(byte).unwrap() > 2 {
            map.entry(&bytes[j]).and_modify(|x| *x -= 1);
            j += 1;
            win -= 1;
        }

        if win > res {
            res = win;
        }
    }

    return res;
}

// Translated implementation of the provided solution
fn maximum_length_substring_res(s: String) -> i32 {
    let mut max = 1;
    let mut counter: HashMap<&u8, i32> = HashMap::new();

    let bytes = s.as_bytes();

    let mut l = 0;
    let mut r = 0;

    counter.insert(&bytes[0], 1);

    while r < bytes.len() - 1 {
        r += 1;

        counter
            .entry(&bytes[r])
            .and_modify(|x| *x += 1)
            .or_insert(1);

        while *counter.get(&bytes[r]).unwrap() == 3 {
            counter.entry(&bytes[l]).and_modify(|x| *x -= 1);
            l += 1;
        }

        max = max.max(r - l + 1);
    }

    return max as i32;
}
