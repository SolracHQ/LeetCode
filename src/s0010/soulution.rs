use crate::common::Solution;

use super::regex::Pattern;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let pattern = Pattern::new(&p);
        pattern.is_match(&s)
    }
}