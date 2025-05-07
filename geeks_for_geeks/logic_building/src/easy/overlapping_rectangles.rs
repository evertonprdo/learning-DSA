// Overlapping Rectangles: https://www.geeksforgeeks.org/find-two-rectangles-overlap/

pub struct Point {
    x: i32,
    y: i32,
}
pub struct Rectangle {
    tl: Point,
    br: Point,
}
impl Rectangle {
    pub fn from(tl: Point, br: Point) -> Self {
        Rectangle { tl, br }
    }
}

pub struct Solution {}
impl Solution {
    pub fn do_overlap(first: &Rectangle, second: &Rectangle) -> bool {
        if first.tl.x > second.br.x || second.tl.x > first.br.x {
            return false;
        }

        if first.br.y > second.tl.y || second.br.y > first.tl.y {
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
        let a = Rectangle::from(Point { x: 0, y: 10 }, Point { x: 10, y: 0 });
        let b = Rectangle::from(Point { x: 5, y: 5 }, Point { x: 15, y: 5 });

        assert!(Solution::do_overlap(&a, &b));
    }

    #[test]
    fn example_02() {
        let a = Rectangle::from(Point { x: 0, y: 10 }, Point { x: 10, y: 0 });
        let b = Rectangle::from(Point { x: -10, y: 5 }, Point { x: -1, y: 0 });

        assert!(!Solution::do_overlap(&a, &b));
    }
}
