// https://leetcode.com/problems/longest-common-prefix/
use crate::common::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter().skip(1).fold(strs[0].clone(), |common_prefix, next_string| {
            common_prefix // Get the current common prefix
            .chars() // Convert to charaters
            .zip(next_string.chars()) // Make pairs with the next string
            .take_while(|(c1,c2)| c1 == c2) // Take only the new common prefix
            .map(|(c, _)| c) // Discard one ement of th pairs
            .collect() // return the new common prefix
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                format!("flower"),
                format!("flow"),
                format!("flight")
            ]),
            format!("fl")
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                format!("dog"),
                format!("racecar"),
                format!("car")
            ]),
            format!("")
        )
    }
}
