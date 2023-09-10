/**
 * # 5. Longest Palindromic Substring - https://leetcode.com/problems/longest-palindromic-substring/
 * Given a string s, returns the longest palindromic substring in s.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len(); // get length of input string
        if n < 2 { // if input string has length less than 2, it is already a palindrome, so return it
            return s;
        }

        // Initialize a 2D boolean table to mark whether substrings are palindromes or not
        let mut palindromes_2d_array = vec![vec![false; n]; n];

        // Initialize the diagonal of the table to true
        // since single characters are palindromes
        for i in 0..n {
            palindromes_2d_array[i][i] = true;
        }

        let mut start = 0;
        let mut end = 0;

        // Build up the table using dynamic programming
        for j in 1..n { // iterate through all possible ending indices of a substring
            for i in 0..j { // iterate through all possible starting indices of a substring
                if s.as_bytes()[i] != s.as_bytes()[j] {
                    // if the starting and ending characters of the substring do not match
                    // the substring is not a palindrome
                    palindromes_2d_array[i][j] = false;
                } else if j - i < 3 {
                    // if the substring has length less than 3 the substring is a palindrome
                    palindromes_2d_array[i][j] = true;
                } else {
                    // check if the substring without its starting and ending characters
                    // is a palindrome
                    palindromes_2d_array[i][j] = palindromes_2d_array[i+1][j-1];
                }

                // if the current substring is a palindrome and its length is greater than
                // the current longest palindrome
                if palindromes_2d_array[i][j] && j - i > end - start {
                    // update starting index of longest palindrome
                    start = i;
                    // update ending index of longest palindrome
                    end = j;
                }
            }
        }

        String::from_utf8_lossy(&s.as_bytes()[start..end+1]).to_string() // return longest palindrome as a string
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_owned()),
            "bab"
        )
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_owned()),
            "bb"
        )
    }
    #[test]
    fn example_3() {
        assert_eq!(
            Solution::longest_palindrome("abb".to_owned()),
            "bb"
        )
    }
}