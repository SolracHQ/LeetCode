/*
 * Problem: 35. Search Insert Position
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. 
 * If not, return the index where it would be if it were inserted in order.
 * You must write an algorithm with O(log n) runtime complexity.
*/
struct Solution;

impl Solution {
    // O(log n)
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            // >> 1 is the same as / 2, but faster (only for positive int numbers)
            let mid = left + ((right - left) >> 1);
            match nums[mid] {
                x if x < target => left = mid + 1,
                x if x > target => right = mid,
                _ => return mid as _,
            }
        }
        left as _
    }
}

mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4)
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0)
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::search_insert(vec![1], 0), 0)
    }
}