/**
* 136. Single Number
*
* Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

*
* Example 1:
* Input: nums = [2,2,1]
* Output: 1
*
* Example 2:
* Input: nums = [4,1,2,1,2]
* Output: 4
*
* Example 3:
* Input: nums = [1]
* Output: 1

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            // a xor b xor b = a, so with a = 0, we have that b xor b = 0, and since all numbers appear twice except one, the result will be that single number
            result ^= num;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [2,2,1]
        // Expected: 1
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn example_2() {
        // Input: nums = [4,1,2,1,2]
        // Expected: 4
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn example_3() {
        // Input: nums = [1]
        // Expected: 1
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
