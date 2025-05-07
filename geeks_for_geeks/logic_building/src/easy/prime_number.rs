// Prime number: https://www.geeksforgeeks.org/introduction-to-primality-test-and-school-method/

pub struct Solution {}
impl Solution {
    pub fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 3..n {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

pub struct SqrtSolution {}
impl SqrtSolution {
    pub fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }

        let sqrt = (n as f32).sqrt() as u32;
        for i in 2..=sqrt {
            if n % i == 0 {
                return false;
            }
        }

        true
    }
}

pub struct SixKSolution {}
impl SixKSolution {
    pub fn is_prime(n: u32) -> bool {
        if n == 2 || n == 3 {
            return true;
        }

        if n <= 1 || n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 11;
        assert!(Solution::is_prime(n));
        assert!(SqrtSolution::is_prime(n));
    }

    #[test]
    fn example_02() {
        let n = 15;
        assert!(!Solution::is_prime(n));
        assert!(!SqrtSolution::is_prime(n));
    }

    #[test]
    fn example_03() {
        let n = 1;
        assert!(!Solution::is_prime(n));
        assert!(!SqrtSolution::is_prime(n));
    }

    #[test]
    fn example_04() {
        let n = 2;
        assert!(Solution::is_prime(n));
        assert!(SqrtSolution::is_prime(n));
    }
}
