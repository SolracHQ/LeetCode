use crate::common::Solution;

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
        _ => unreachable!("This will never happens")
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {return vec![];}
        let sets: Vec<&[char]> = digits
            .as_bytes()
            .iter()
            .map(loopback)
            .collect();
        let total: usize = sets.iter().map(|s| s.len()).sum();
        let result = sets.iter().fold(vec![String::new()], |res, set| {
            (0..(res.len()*set.len()))
            .map(|i| format!("{}{}", res[i/set.len()], set[i%set.len()]))
            .collect()
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string_vec(vec: Vec<&str>) -> Vec<String> {
        vec
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::letter_combinations(format!("23")),
            to_string_vec(vec!["ad","ae","af","bd","be","bf","cd","ce","cf"])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::letter_combinations(String::new()),
            to_string_vec(vec![])
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::letter_combinations(format!("2")),
            to_string_vec(vec!["a","b","c"])
        )
    }
}