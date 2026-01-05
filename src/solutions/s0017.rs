/**
* 17. Letter Combinations of a Phone Number
*
* Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.

*
* Example 1:
* Input: digits = "23"
* Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
*
* Example 2:
* Input: digits = "2"
* Output: ["a","b","c"]

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
fn loopback(d: &u8) -> &'static [char] {
    match d {
        b'2' => &['a', 'b', 'c'],
        b'3' => &['d', 'e', 'f'],
        b'4' => &['g', 'h', 'i'],
        b'5' => &['j', 'k', 'l'],
        b'6' => &['m', 'n', 'o'],
        b'7' => &['p', 'q', 'r', 's'],
        b'8' => &['t', 'u', 'v'],
        b'9' => &['w', 'x', 'y', 'z'],
        _ => unreachable!("This will never happens"),
    }
}

#[cfg(test)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // If the input is empty, return an empty vector
        if digits.len() == 0 {
            return vec![];
        }

        // If the input is empty, return an empty vector
        let sets: Vec<&[char]> = digits.as_bytes().iter().map(loopback).collect();

        // Generate all possible combinations of letters
        let result = sets.iter().fold(vec![String::new()], |res, set| {
            (0..(res.len() * set.len()))
                .map(|i| format!("{}{}", res[i / set.len()], set[i % set.len()]))
                .collect()
        });
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn normalize(mut v: Vec<String>) -> Vec<String> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_1() {
        // Input: digits = "23"
        // Expected: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
        let result = Solution::letter_combinations("23".to_string());
        let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn example_2() {
        // Input: digits = "2"
        // Expected: ["a","b","c"]
        let result = Solution::letter_combinations("2".to_string());
        let expected = vec!["a", "b", "c"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(normalize(result), normalize(expected));
    }
}
