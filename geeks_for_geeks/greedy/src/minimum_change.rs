// Minimum number of coin: https://www.geeksforgeeks.org/introduction-to-greedy-algorithm-data-structures-and-algorithm-tutorials/

pub struct Solution {}
impl Solution {
    // The fn assumes that coins are sorted in ascending order
    pub fn minimum_change(coins: &[u32], mut change: u32) -> (Vec<u32>, u32) {
        let mut count = vec![0; coins.len()];
        let mut total = 0;

        let mut i = coins.len();
        while change > 0 {
            i -= 1;

            let coin = coins[i];
            let n = change / coin;

            change -= n * coin;
            total += n;

            count[i] = n;
        }

        (count, total)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let coins = [1, 2, 5, 10];
        let change = 39;

        let expected = (vec![0, 2, 1, 3], 6);

        assert_eq!(Solution::minimum_change(&coins, change), expected);
    }

    #[test]
    fn example_02() {
        let coins = [1, 5, 10, 25];
        let change = 20;

        let expected = (vec![0, 0, 2, 0], 2);

        assert_eq!(Solution::minimum_change(&coins, change), expected);
    }

    #[test]
    fn example_03() {
        let coins = [1, 2, 5, 10, 25, 50, 100];
        let change = 73;

        let expected = (vec![1, 1, 0, 2, 0, 1, 0], 5);

        assert_eq!(Solution::minimum_change(&coins, change), expected);
    }
}
