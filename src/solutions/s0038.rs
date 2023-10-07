/*
 * Problem: 38. Count and Say
 * The count-and-say sequence is the sequence of integers with the first five terms as following:
 * - countAndSay(1) = "1"
 * - countAndSay(n) is the way you would "say" the digit string from countAndSay(n-1), which is then converted into a different digit string.
 * To determine how you "say" a digit string, split it into the minimal number of substrings such that each substring contains exactly one unique digit.
 * Then for each substring, say the number of digits, then say the digit.
 * Finally, concatenate every said digit.
*/
struct Solution;

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

mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let n = 1;
        let answer = "1".to_string();
        assert_eq!(Solution::count_and_say(n), answer);
    }

    #[test]
    fn example2() {
        let n = 4;
        let answer = "1211".to_string();
        assert_eq!(Solution::count_and_say(n), answer);
    }
}
