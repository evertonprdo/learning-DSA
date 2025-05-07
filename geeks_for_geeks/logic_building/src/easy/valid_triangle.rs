// Valid Triangle: https://www.geeksforgeeks.org/check-whether-triangle-valid-not-sides-given/

pub struct Solution {}
impl Solution {
    pub fn check_valid(a: u32, b: u32, c: u32) -> bool {
        if a + b <= c || a + c <= b || b + c <= a {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let (a, b, c) = (7, 10, 5);

        assert!(Solution::check_valid(a, b, c));
    }

    #[test]
    fn example_02() {
        let (a, b, c) = (1, 10, 12);

        assert!(!Solution::check_valid(a, b, c));
    }
}
