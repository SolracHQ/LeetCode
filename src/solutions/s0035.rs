/**
* 35. Search Insert Position
*
* Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

*
* Example 1:
* Input: nums = [1,3,5,6], target = 5
* Output: 2
*
* Example 2:
* Input: nums = [1,3,5,6], target = 2
* Output: 1
*
* Example 3:
* Input: nums = [1,3,5,6], target = 7
* Output: 4

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
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

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [1,3,5,6], target = 5
        // Expected: 2
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    }

    #[test]
    fn example_2() {
        // Input: nums = [1,3,5,6], target = 2
        // Expected: 1
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
    }

    #[test]
    fn example_3() {
        // Input: nums = [1,3,5,6], target = 7
        // Expected: 4
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    }
}
