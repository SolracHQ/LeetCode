use std::collections::HashMap;
use crate::common::Solution;

/**
 * 1. Two Sum
 *
 * Given an array of integers `nums` and an integer `target`, return indices of the two numbers
 * such that they add up to `target`.
 *
 * You may assume that each input would have exactly one solution, and you may not use the same
 * element twice.
 *
 * You can return the answer in any order.
 */
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a new empty HashMap to store complements of the target
        let mut complements = HashMap::new();

        // Iterate over the vector "nums" and enumerate the values with their index
        for (i, num) in nums.iter().enumerate() {
            // Calculate the complement of the current value
            let complement = target - num;

            // If the complement is in the complements HashMap, return a vector with the indices
            // of the current value and its complement
            if let Some(&j) = complements.get(&complement) {
                return vec![j as i32, i as i32];
            }

            // Otherwise, insert the current value and its index into the complements HashMap
            complements.insert(num, i);
        }

        // If no two values add up to the target, return an empty vector
        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;

    #[test]
    fn example_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2,7,11,15], 9));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1,2], Solution::two_sum(vec![3,2,4], 6));
    }
    #[test]
    fn example_3() {
        assert_eq!(vec![0,1], Solution::two_sum(vec![3,3], 6));
    }
}