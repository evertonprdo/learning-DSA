// Check Power: https://www.geeksforgeeks.org/check-if-a-number-is-power-of-another-number/

pub struct Solution {}
impl Solution {
    pub fn is_power(x: u32, y: u32) -> bool {
        if x == 1 {
            return y == 1;
        }

        let mut power = 1;
        while power < y {
            power *= x;
        }

        power == y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let x = 10;
        let y = 1;

        assert!(Solution::is_power(x, y));
    }

    #[test]
    fn example_02() {
        let x = 10;
        let y = 1000;

        assert!(Solution::is_power(x, y));
    }

    #[test]
    fn example_03() {
        let x = 10;
        let y = 1001;

        assert!(!Solution::is_power(x, y));
    }
}
