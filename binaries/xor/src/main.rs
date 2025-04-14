// 268. Missing Number: https://leetcode.com/problems/missing-number

fn main() {
    let nums = vec![0, 1];
    println!("{}", Solution::missing_number(nums));
}

// XOR operator and the property of x ^ x = 0
struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut i = 1; // avoid conversions to i32

        for n in nums {
            r ^= n ^ i;
            i += 1;
        }

        r
    }
}
