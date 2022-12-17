// https://leetcode.com/problems/longest-substring-without-repeating-characters/

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut biggest = 0;
    let mut sub = String::from("");
    for char in s.chars() {
        let index_opt = sub.as_bytes().iter().enumerate().filter(|(i,c)| **c as char == char).next();
        if index_opt.is_some() {
            biggest = i32::max(biggest, sub.len() as i32);
            sub = sub[index_opt.unwrap().0 + 1..].to_owned()
        }
        sub.push(char);
    }
    biggest = i32::max(biggest, sub.len() as i32);
    biggest as i32
}

mod test {

    use crate::s0003::length_of_longest_substring;    

    #[test]
    fn example_1() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_owned()));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, length_of_longest_substring("bbbbb".to_owned()));
    }
}