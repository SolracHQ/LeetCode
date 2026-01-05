/**
* 38. Count and Say
*
* The count-and-say sequence is a sequence of digit strings defined by the recursive formula.

*
* Example 1:
* Input: n = 4
* Output: "1211"
*
* Example 2:
* Input: n = 1
* Output: "1"

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = vec![1u8];
        for _ in 1..n {
            let mut buffer = vec![];
            let (mut current, mut count) = (result[0], 0);
            for n in result {
                if n == current {
                    count += 1;
                } else {
                    buffer.extend_from_slice(&[count, current]);
                    current = n;
                    count = 1;
                }
            }
            buffer.extend_from_slice(&[count, current]);
            result = buffer;
        }
        result.into_iter().map(|x| (x + b'0') as char).collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: n = 4
        // Expected: "1211"
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
    }

    #[test]
    fn example_2() {
        // Input: n = 1
        // Expected: "1"
        assert_eq!(Solution::count_and_say(1), "1".to_string());
    }
}
