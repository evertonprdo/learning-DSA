// Distance between two points: https://www.geeksforgeeks.org/program-calculate-distance-two-points/

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Solution {}
impl Solution {
    pub fn distance(a: &Point, b: &Point) -> f64 {
        let x = a.x - b.x;
        let y = a.y - b.y;

        (x * x + y * y).sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let a = Point { x: 3.0, y: 4.0 };
        let b = Point { x: 7.0, y: 7.0 };

        let output = 5.0;
        assert_eq!(Solution::distance(&a, &b), output);
    }

    #[test]
    fn example_02() {
        let a = Point { x: 3.0, y: 4.0 };
        let b = Point { x: 4.0, y: 3.0 };

        let output = 1.4142135623730951;
        assert_eq!(Solution::distance(&a, &b), output);
    }
}
