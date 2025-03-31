// 387. First Unique Character in a String: https://leetcode.com/problems/first-unique-character-in-a-string/
use std::{collections::HashMap, i32};

fn main() {
    println!("{}", first_uniq_char("leetcode".to_string()));
    println!("{}", first_uniq_char_2("leetcode".to_string()));
    println!("{}", first_uniq_char_3("leetcode".to_string()));
}

// My first attempt, but in the end it was almost the same as the solution provided
fn first_uniq_char(s: String) -> i32 {
    let mut map: HashMap<&u8, (usize, i32)> = HashMap::new();
    let bytes = s.as_bytes();

    for (i, b) in bytes.iter().enumerate() {
        map.entry(b)
            .and_modify(|x| *x = (i, x.1 + 1))
            .or_insert((i, 1));
    }

    for b in bytes {
        let (i, c) = map.get(b).unwrap();
        if *c == 1 {
            return *i as i32;
        }
    }

    -1
}

// Also me trying hard to get 0ms solution
fn first_uniq_char_2(s: String) -> i32 {
    let mut r: Vec<usize> = Vec::new();
    let mut map: HashMap<&u8, usize> = HashMap::new();
    let bytes = s.as_bytes();

    for (i, b) in bytes.iter().enumerate() {
        map.entry(b)
            .and_modify(|x| {
                if let Ok(v) = r.binary_search(x) {
                    r.remove(v);
                }
            })
            .or_insert_with(|| {
                r.push(i);
                i
            });
    }

    if let Some(v) = r.first() {
        *v as i32
    } else {
        -1
    }
}

// The minified 0ms solution with O(1) space complexity
// https://leetcode.com/problems/first-unique-character-in-a-string/solutions/6055949/video-short-and-simple-o-n-3-approaches/
fn first_uniq_char_3(s: String) -> i32 {
    // Constraints: 1 <= s.length <= 10^5 (i.e, 100,000); so the last possible index is 99,999
    let mut res = 100_000;

    for c in 'a'..='z' {
        s.find(c).map(|left| {
            if s.rfind(c) == Some(left) {
                res = res.min(left);
            };
        });
    }

    // If res remains 100,000, it means there is no valid output, so return -1
    if res == 100_000 { -1 } else { res as i32 }
}
