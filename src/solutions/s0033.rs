/*
 * Problem: 33. Search in Rotated Sorted Array
 * There is an integer array nums sorted in ascending order (with distinct values).
 * Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
 * Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
 * You must write an algorithm with O(log n) runtime complexity.
*/
struct Solution;

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

mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::search(vec![1], 0), -1)
    }
}
