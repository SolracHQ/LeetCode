/**
* 137. Single Number II
*
* Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it. You must implement a solution with a linear runtime complexity and use only constant extra space.

*
* Example 1:
* Input: nums = [2,2,3,2]
* Output: 3
*
* Example 2:
* Input: nums = [0,1,0,1,0,1,99]
* Output: 99

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // Count the number of set bits at each position across all numbers.
        // Using unsigned shift to avoid issues with sign-extension.
        let mut bits = [0u32; 32];
        for num in nums {
            let n = num as u32;
            for i in 0..32 {
                bits[i] += (n >> i) & 1;
            }
        }

        // Reconstruct the unique number from bit counts modulo 3.
        let mut result: i32 = 0;
        for i in 0..32 {
            if bits[i] % 3 != 0 {
                result |= 1i32 << i;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [2,2,3,2]
        // Expected: 3
        assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
    }

    #[test]
    fn example_2() {
        // Input: nums = [0,1,0,1,0,1,99]
        // Expected: 99
        assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    }
}
