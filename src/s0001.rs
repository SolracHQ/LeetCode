use std::collections::HashMap;

use crate::common::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lookup_table: HashMap<_, _> = nums
            .iter()
            .enumerate()
            .map(|(index, value)| (value.clone(), index.clone()))
            .collect();
        for (index, number) in nums.iter().enumerate() {
            if lookup_table.contains_key(&(target - *number)) && lookup_table[&(target - *number)] != index {
                return vec![index as i32, lookup_table[&(target - *number)] as i32];
            }
        }
        vec![]
    }
}
