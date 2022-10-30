// https://leetcode.com/problems/regular-expression-matching/
use crate::common::Solution;

#[derive(Debug)]
struct Pattern {
    states: Vec<State>,
}

#[derive(Debug)]
enum State {
    Char {
        chr: u8,
        multiple: bool,
        required: i8,
    },
    Dot {
        multiple: bool,
    },
}

impl State {
    fn new(chr: u8, multiple: bool) -> Self {
        match chr {
            b'.' => Self::Dot { multiple },
            _ => Self::Char {
                chr,
                multiple,
                required: 1 - multiple as i8,
            }
        }
    }
    fn match_chr(&self, c: u8) -> bool {
        match self {
            State::Char { chr, .. } => *chr == c,
            State::Dot { .. } => true,
        }
    }
    fn multiple(&self) -> bool {
        match self {
            Self::Char { multiple, .. } => *multiple,
            Self::Dot { multiple } => *multiple,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Char { chr: l_chr, .. }, Self::Char { chr: r_chr, .. }) => l_chr == r_chr,
            (
                Self::Dot {
                    multiple: l_multiple,
                },
                Self::Dot {
                    multiple: r_multiple,
                },
            ) => l_multiple == r_multiple,
            _ => false,
        }
    }
}

impl Pattern {
    fn new(pattern: &[u8]) -> Self {
        let mut states = vec![];
        let mut index = 0;
        while index < pattern.len() {
            let _multiple = *pattern.get(index + 1).unwrap_or(&b'?') == b'*';
            match pattern[index] {
                b'.' => states.push(State::Dot { multiple: _multiple }),
                _ => {
                    if let Some(State::Char {
                        chr,
                        multiple,
                        required,
                    }) = &mut states.last_mut()
                    {
                        if *chr == pattern[index] {
                            *required += 1;
                            if _multiple {
                                *required -= 1;
                                *multiple = true;
                            }
                        } else {
                            states.push(State::new(pattern[index], _multiple));
                        }
                    } else {
                        states.push(State::new(pattern[index], _multiple));
                    }
                }
            };
            index += 1 + _multiple as usize
        }
        Pattern { states }
    }

    fn match_str(&mut self, s: &[u8]) -> bool {
        let (mut pidx, mut sidx) = (0, 0);
        while pidx < self.states.len() && sidx < s.len() {
            if self.states[pidx].multiple() {
                if !self.states[pidx].match_chr(s[sidx]) {
                    if let State::Char { chr: _, multiple: _, required } = self.states[pidx] {
                        if required > 0 {return false;}
                    }
                    pidx += 1;
                } else {
                    if let Some(State::Char { chr: _, multiple: _, required }) = self.states.get_mut(pidx) {
                        *required -= 1;
                    }
                    sidx += 1;
                }
            } else {
                if !self.states[pidx].match_chr(s[sidx]) {
                    return false;
                }
                if let Some(State::Char { chr: _, multiple: _, required }) = self.states.get_mut(pidx) {
                    *required -= 1;
                    sidx += 1;
                    if *required <= 0 {
                        pidx += 1;
                    }
                } else {
                    pidx += 1;
                    sidx += 1;
                }
            }
        }
        while pidx < self.states.len() {
            if self.states[pidx].multiple() {
                if let Some(State::Char { chr: _, multiple: _, required }) = self.states.get(pidx) {
                    if *required > 0{ break;}
                }
                pidx += 1;
            } else {
                break;
            }
        }
        pidx >= self.states.len() && sidx >= s.len()
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut pattern = Pattern::new(p.as_bytes());
        println!("{pattern:?}");
        pattern.match_str(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_match(format!("aa"), format!("a")), false)
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_match(format!("aa"), format!("a*")), true)
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::is_match(format!("ab"), format!(".*")), true)
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::is_match(format!("aab"), format!("c*a*b")), true)
    }
    #[test]
    fn example5() {
        assert_eq!(Solution::is_match(format!("aaa"), format!("a*a")), true)
    }
    #[test]
    fn example6() {
        assert_eq!(Solution::is_match(format!("aa"), format!("aa")), true)
    }
    #[test]
    fn example7() {
        assert_eq!(Solution::is_match(format!("aaa"), format!("ab*a*c*a")), true)
    }
}
