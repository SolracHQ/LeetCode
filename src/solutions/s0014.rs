/**
* 14. Longest Common Prefix
*
* Write a function to find the longest common prefix string amongst an array of strings. If there is no common prefix, return an empty string.

*
* Example 1:
* Input: strs = ["flower","flow","flight"]
* Output: "fl"
*
* Example 2:
* Input: strs = ["dog","racecar","car"]
* Output: ""

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter()
            .skip(1)
            .fold(strs[0].clone(), |common_prefix, next_string| {
                common_prefix // Get the current common prefix
                    .chars() // Convert to charaters
                    .zip(next_string.chars()) // Make pairs with the next string
                    .take_while(|(c1, c2)| c1 == c2) // Take only the new common prefix
                    .map(|(c, _)| c) // Discard one ement of th pairs
                    .collect() // return the new common prefix
            })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: strs = ["flower","flow","flight"]
        // Expected: "fl"
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
    }

    #[test]
    fn example_2() {
        // Input: strs = ["dog","racecar","car"]
        // Expected: ""
        assert_eq!(
            "".to_string(),
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
    }
}
