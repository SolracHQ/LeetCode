/**
* 33. Search in Rotated Sorted Array
*
* There is an integer array nums sorted in ascending order (with distinct values), which is rotated at an unknown pivot.

*
* Example 1:
* Input: nums = [4,5,6,7,0,1,2], target = 0
* Output: 4
*
* Example 2:
* Input: nums = [4,5,6,7,0,1,2], target = 3
* Output: -1
*
* Example 3:
* Input: nums = [1], target = 0
* Output: -1

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let middle = (left + right) / 2;
            if nums[middle] == target {
                return middle as i32;
            }
            // If nums[left] <= nums[middle], the left half is sorted
            if nums[left] <= nums[middle] {
                if nums[left] <= target && target < nums[middle] {
                    // If nums[left] <= target < nums[middle], the target is in the left half
                    right = middle - 1;
                } else {
                    // Otherwise, the target is in the right half
                    left = middle + 1;
                }
            } else {
                // Otherwise, the right half is sorted
                if nums[middle] < target && target <= nums[right] {
                    // If nums[middle] < target <= nums[right], the target is in the right half
                    left = middle + 1;
                } else {
                    // Otherwise, the target is in the left half
                    right = middle - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [4,5,6,7,0,1,2], target = 0
        // Expected: 4
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn example_2() {
        // Input: nums = [4,5,6,7,0,1,2], target = 3
        // Expected: -1
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }

    #[test]
    fn example_3() {
        // Input: nums = [1], target = 0
        // Expected: -1
        assert_eq!(-1, Solution::search(vec![1], 0));
    }
}
