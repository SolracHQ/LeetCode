/**
* 11. Container With Most Water
*
* You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

*
* Example 1:
* Input: height = [1,8,6,2,5,4,8,3,7]
* Output: 49
*
* Example 2:
* Input: height = [1,1]
* Output: 1

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
        // Input: height = [1,8,6,2,5,4,8,3,7]
        // Expected: 49
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn example_2() {
        // Input: height = [1,1]
        // Expected: 1
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }
}
