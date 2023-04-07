/**
 * # 11. Container With Most Water - https://leetcode.com/problems/container-with-most-water/
 * You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
 * Find two lines that together with the x-axis form a container, such that the container contains the most water.
 * Return the maximum amount of water a container can store.
 * Notice that you may not slant the container.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
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

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1)
    }
}
