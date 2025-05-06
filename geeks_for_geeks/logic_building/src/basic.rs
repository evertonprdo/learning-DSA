pub struct Solution {}
impl Solution {
    // https://www.geeksforgeeks.org/check-whether-given-number-even-odd/
    pub fn is_even(n: i32) -> bool {
        !(n & 1 == 1)
    }

    // https://www.geeksforgeeks.org/program-to-print-multiplication-table-of-a-number/
    pub fn print_table(n: i32) {
        for i in 1..=10 {
            println!("{n} * {i} = {}", n * i);
        }
    }

    // https://www.geeksforgeeks.org/program-find-sum-first-n-natural-numbers/
    pub fn find_sum(n: u32) -> u32 {
        let mut sum = 0;
        for i in 0..=n {
            sum += i;
        }
        sum
    }

    // https://www.geeksforgeeks.org/swap-two-numbers/
    pub fn swap(a: &mut i32, b: &mut i32) {
        (*b, *a) = (*a, *b)
    }

    // https://www.geeksforgeeks.org/find-number-closest-n-divisible-m/
    pub fn closest_number(n: i32, m: i32) -> i32 {
        let q = n / m;
        let n1 = m * q;

        let n2 = if n * m > 0 { m * (q + 1) } else { m * (q - 1) };

        if (n - n1).abs() < (n - n2).abs() {
            n1
        } else {
            n2
        }
    }

    // https://www.geeksforgeeks.org/the-dice-problem/
    pub fn dice_problem(n: u8) -> u8 {
        7 - n
    }

    // https://www.geeksforgeeks.org/nth-term-of-ap-from-first-two-terms/
    pub fn nth_term_of_ap(a: i32, b: i32, n: i32) -> i32 {
        a + (n - 1) * (b - a)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(!Solution::is_even(-1));
        assert!(Solution::is_even(0));
        assert!(!Solution::is_even(1));
        assert!(Solution::is_even(2));
        assert!(!Solution::is_even(3));
    }

    #[test]
    fn test_find_sum() {
        assert_eq!(Solution::find_sum(0), 0);
        assert_eq!(Solution::find_sum(1), 1);
        assert_eq!(Solution::find_sum(2), 3);
        assert_eq!(Solution::find_sum(3), 6);
        assert_eq!(Solution::find_sum(5), 15);
    }

    #[test]
    fn test_swap() {
        let (mut a, mut b) = (2, 3);
        Solution::swap(&mut a, &mut b);
        assert_eq!(a, 3);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_closest_number() {
        let n = 13;
        let m = 4;
        let output = 12;

        assert_eq!(Solution::closest_number(n, m), output);

        let n = -15;
        let m = 6;
        let output = -18;

        assert_eq!(Solution::closest_number(n, m), output);
    }

    #[test]
    fn test_dice_problem() {
        assert_eq!(Solution::dice_problem(2), 5);
        assert_eq!(Solution::dice_problem(1), 6);
        assert_eq!(Solution::dice_problem(3), 4);
    }

    #[test]
    fn test_nth_term_of_ap() {
        assert_eq!(Solution::nth_term_of_ap(2, 3, 4), 5);
        assert_eq!(Solution::nth_term_of_ap(1, 3, 10), 19);
    }
}
