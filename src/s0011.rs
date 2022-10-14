use crate::common::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let (mut left, mut right, mut area) = (0, height.len() - 1, 0);
        while left < right {
            let min_h = min(height[left], height[right]);
            area = max(area, (right - left) as i32 * min_h);
            // Skip right bars smaller than actual
            while (min_h >= height[right]) && (left < right) {
                right -= 1;
            } 
            // Skip left bars smaller than actual
            while (min_h >= height[left]) && (left < right) {
                left += 1;
            }
        }
        area
    }
}

mod test {
    use crate::common::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1)
    }
}
