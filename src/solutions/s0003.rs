/**
 * # 3. Longest Substring Without Repeating Characters - https://leetcode.com/problems/longest-substring-without-repeating-characters/
 * Given a string s, find the length of the longest
 * substring without repeating characters.
 * Use the use statement and ````` to format.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // initialize variable to keep track of longest substring length
        let mut biggest = 0;
        // initialize boolean array to keep track of characters in current substring
        let mut sub = [false; 128];
        // get bytes of input string
        let bytes = s.as_bytes();
        // initialize variable to keep track of starting index of current substring
        let mut start = 0;
        // iterate through each byte in the input string
        for end in 0..bytes.len() {
            // get current byte as usize
            let byte = bytes[end] as usize;
            while sub[byte] {  // if the current byte is already in the current substring
                // remove the byte at the starting index of the current substring
                sub[bytes[start] as usize] = false;
                start += 1;  // move the starting index of the current substring forward by 1
            }
            // add the current byte to the current substring
            sub[byte] = true;
            // update longest substring length if necessary
            biggest = i32::max(biggest, (end - start + 1) as i32);
        }
        biggest
    }
}


#[cfg(test)]
mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_owned())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_owned())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_owned())
        )
    }
}