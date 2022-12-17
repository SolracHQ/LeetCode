use super::super::common::Solution;

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