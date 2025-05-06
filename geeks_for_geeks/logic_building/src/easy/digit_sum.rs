// Sum of Digits of a Number: https://www.geeksforgeeks.org/program-for-sum-of-the-digits-of-a-given-number/

pub struct Solution {}
impl Solution {
    pub fn sum_of_digits(mut n: u32) -> u32 {
        let mut res = 0;

        while n > 0 {
            let digit = n % 10;
            n /= 10;

            res += digit;
        }

        res
    }

    pub fn sum_of_digits_str(n: &str) -> u32 {
        let mut res = 0;
        for byte in n.as_bytes() {
            res += (byte - b'0') as u32
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 687;
        let output = 21;

        assert_eq!(Solution::sum_of_digits(n), output);
    }

    #[test]
    fn example_02() {
        let n = 12;
        let output = 3;

        assert_eq!(Solution::sum_of_digits(n), output);
    }

    #[test]
    fn example_03() {
        let n = "123456789123456789123422";
        let output = 104;

        assert_eq!(Solution::sum_of_digits_str(n), output);
    }
}
