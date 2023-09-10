#[derive(Debug, Clone)]
enum Kind {
    WildCard,
    Element(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Quantifier {
    One,
    ZeroOrMore,
}

#[derive(Debug, Clone)]
struct State {
    kind: Kind,
    quantifier: Quantifier,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    idx: usize,
    states: Vec<State>,
}

impl Pattern {
    pub fn new(re: &str) -> Pattern {
        let re = re.as_bytes();
        let mut states = vec![];
        for r in re {
            match r {
                b'a'..=b'z' => states.push(State {
                    kind: Kind::Element(*r as char),
                    quantifier: Quantifier::One,
                }),
                b'.' => states.push(State {
                    kind: Kind::WildCard,
                    quantifier: Quantifier::One,
                }),
                b'*' => states.last_mut().unwrap().quantifier = Quantifier::ZeroOrMore,
                _ => {
                    unreachable!("Tis will never happen");
                }
            }
        }
        Pattern { idx: 0, states }
    }

    pub fn re_match(&mut self, text: &str) -> bool {
        is_match_priv(self,text, 0)
    }
}

impl State {
    fn match_st(&self, ch: char) -> bool {
        match self.kind {
            Kind::WildCard => true,
            Kind::Element(c) => c == ch,
        }
    }
}

fn is_match_priv(pattern: &mut Pattern, text: &str, index: usize) -> bool {
    let mut index = index;
    let chars = text.chars().collect::<Vec<_>>();
    loop {
        if pattern.idx >= pattern.states.len() {
            return index == chars.len();
        }
        let current_state = &pattern.states[pattern.idx];
        if index == chars.len() {
            return pattern.states[pattern.idx..].iter().all(|e| e.quantifier == Quantifier::ZeroOrMore);
        }
        match current_state.quantifier {
            Quantifier::One => {
                if !current_state.match_st(chars[index]) {
                    return false;
                }
                index += 1;
                pattern.idx += 1;
            },
            Quantifier::ZeroOrMore => {
                let mut fake = vec![current_state.clone()];
                fake[0].quantifier = Quantifier::One;
                fake.extend_from_slice(&pattern.states[pattern.idx..]);
                pattern.idx += 1;
                if is_match_priv(&mut Pattern { idx: 0, states: fake }, text, index) {
                    return true;
                }
            },
        }
    }
}
