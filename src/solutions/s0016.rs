/**
* 16. 3Sum Closest
*
* Given an integer array nums of length n and an integer target, find three integers at distinct indices in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.

*
* Example 1:
* Input: nums = [-1,2,1,-4], target = 1
* Output: 2
*
* Example 2:
* Input: nums = [0,0,0], target = 1
* Output: 0

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // STRATEGY:
        // 1. Sort the array to enable two-pointer technique and pruning optimizations
        // 2. For each number as the first element of the triplet:
        //    a. Use early exit if smallest possible sum exceeds target (no better solutions exist)
        //    b. Use early skip if largest possible sum is below target (try next first element)
        //    c. Use two pointers to efficiently find the closest sum for remaining two elements
        // 3. Track the closest sum found and its distance from target
        //
        // Time Complexity: O(n²) worst case, O(n) best case with early pruning
        // Space Complexity: O(1) excluding sort

        nums.sort_unstable();
        let array_length = nums.len();

        // Track best result as (sum, absolute_difference_from_target)
        let mut best_result = (0, i32::MAX);

        // Try each element as the first number in our triplet
        for first_index in 0..array_length - 2 {
            // OPTIMIZATION 1: Skip duplicate first elements
            // If we already processed triplets starting with value X,
            // processing another triplet with the same starting value X won't yield
            // a different closest sum - we'd just rediscover the same answer
            if first_index > 0 && nums[first_index] == nums[first_index - 1] {
                continue;
            }

            // OPTIMIZATION 2: Early exit if smallest possible sum is already too large
            // Since array is sorted, the three consecutive elements starting
            // at current position form the SMALLEST possible sum for this iteration.
            // If this minimum sum exceeds target, all future sums (with larger indices)
            // will be even larger, so we can stop searching entirely.
            let smallest_possible_sum =
                nums[first_index] + nums[first_index + 1] + nums[first_index + 2];

            if smallest_possible_sum > target {
                let difference = smallest_possible_sum - target;
                if difference < best_result.1 {
                    best_result = (smallest_possible_sum, difference);
                }
                break; // No point continuing - all future sums will be larger
            }

            // OPTIMIZATION 3: Early skip if largest possible sum is still too small
            // The current element plus the two largest elements form the
            // LARGEST possible sum for this first element. If even this maximum is
            // below target, we can't get any closer with this first element, so skip
            // to the next (larger) first element.
            let largest_possible_sum =
                nums[first_index] + nums[array_length - 2] + nums[array_length - 1];

            if largest_possible_sum < target {
                let difference = target - largest_possible_sum;
                if difference < best_result.1 {
                    best_result = (largest_possible_sum, difference);
                }
                continue; // Skip to next first element - can't reach target with current one
            }

            // Standard two-pointer search for the remaining two elements
            let mut left_pointer = first_index + 1;
            let mut right_pointer = array_length - 1;

            while left_pointer < right_pointer {
                let current_sum = nums[first_index] + nums[left_pointer] + nums[right_pointer];

                let difference_from_target = (target - current_sum).abs();

                // Update best result if this sum is closer to target
                if difference_from_target < best_result.1 {
                    best_result = (current_sum, difference_from_target);
                }

                // If we found exact match, return immediately - can't get closer!
                if current_sum == target {
                    return current_sum;
                }

                // Move pointers to get closer to target
                // If sum is too small, move left pointer right to increase sum
                // If sum is too large, move right pointer left to decrease sum
                if current_sum < target {
                    left_pointer += 1;
                } else {
                    right_pointer -= 1;
                }
            }
        }

        // Return the sum portion of our best result
        best_result.0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [-1,2,1,-4], target = 1
        // Expected: 2
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn example_2() {
        // Input: nums = [0,0,0], target = 1
        // Expected: 0
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
