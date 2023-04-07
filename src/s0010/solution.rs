use super::Solution;

use super::regex::Pattern;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut pattern = Pattern::new(&p);
        pattern.re_match(&s)
    }
}