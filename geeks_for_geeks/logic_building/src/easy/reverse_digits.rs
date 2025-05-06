// Reverse digits of a number: https://www.geeksforgeeks.org/write-a-program-to-reverse-digits-of-a-number/

pub struct Solution {}
impl Solution {
    pub fn reverse_digits(mut n: u32) -> u32 {
        let mut digits = Vec::new();
        while n > 0 {
            let digit = n % 10;
            n /= 10;

            digits.push(digit);
        }

        let mut res = 0;
        let mut j = digits.len() as u32;

        for n in digits {
            j -= 1;
            res += n * 10_u32.pow(j); // O-O
        }
        res
    }
}

pub struct ProvidedSolution {}
impl ProvidedSolution {
    pub fn reverse_digits(mut n: u32) -> u32 {
        let mut rev = 0;
        while n > 0 {
            rev = rev * 10 + n % 10;
            n /= 10;
        }
        rev
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 122;
        let output = 221;

        assert_eq!(Solution::reverse_digits(n), output);
        assert_eq!(ProvidedSolution::reverse_digits(n), output);
    }

    #[test]
    fn example_02() {
        let n = 200;
        let output = 2;

        assert_eq!(Solution::reverse_digits(n), output);
        assert_eq!(ProvidedSolution::reverse_digits(n), output);
    }

    #[test]
    fn example_03() {
        let n = 12345;
        let output = 54321;

        assert_eq!(Solution::reverse_digits(n), output);
        assert_eq!(ProvidedSolution::reverse_digits(n), output);
    }
}
