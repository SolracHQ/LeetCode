/**
* 15. 3Sum
*
* Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

*
* Example 1:
* Input: nums = [-1,0,1,2,-1,-4]
* Output: [[-1,-1,2],[-1,0,1]]
*
* Example 2:
* Input: nums = [0,1,1]
* Output: []
*
* Example 3:
* Input: nums = [0,0,0]
* Output: [[0,0,0]]

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res = Vec::new();

        // Sort the input array in non-decreasing order
        nums.sort_unstable();

        for i in 0..n {
            // Skip over duplicates
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = n - 1;
            // Use two pointers to find pairs that sum to -nums[i]
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                    // Skip over duplicates
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    // Normalize a list of combinations by sorting each inner vector and then the outer vector.
    // This makes equality checks order-insensitive.
    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for inner in v.iter_mut() {
            inner.sort_unstable();
        }
        v.sort_unstable();
        v
    }

    #[test]
    fn example_1() {
        // Input: nums = [-1,0,1,2,-1,-4]
        // Expected: [[-1,-1,2],[-1,0,1]]
        let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(
            normalize(result),
            normalize(vec![vec![-1, -1, 2], vec![-1, 0, 1]])
        );
    }

    #[test]
    fn example_2() {
        // Input: nums = [0,1,1]
        // Expected: []
        let result = Solution::three_sum(vec![0, 1, 1]);
        assert!(normalize(result).is_empty());
    }

    #[test]
    fn example_3() {
        // Input: nums = [0,0,0]
        // Expected: [[0,0,0]]
        let result = Solution::three_sum(vec![0, 0, 0]);
        assert_eq!(normalize(result), normalize(vec![vec![0, 0, 0]]));
    }
}
