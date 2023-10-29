/// 39. Combination Sum
/// https://leetcode.com/problems/combination-sum/
///
/// Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
/// The same repeated number may be chosen from candidates unlimited number of times.
/// Note:
/// - All numbers (including target) will be positive integers.
/// - The solution set must not contain duplicate combinations.
/// - For example, given candidate set [2, 3, 6, 7] and target 7,
struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        let mut candidates = candidates;
        candidates.sort_unstable();

        let mut result: Vec<Vec<i32>> = vec![];
        let mut stack = vec![(vec![0], candidates[0])];

        while let Some((mut indices, sum)) = stack.pop() {
            if sum == target {
                // If the sum is equal to the target, add the current combination to the result
                result.push(indices.iter().map(|indice| candidates[*indice]).collect());
                continue;
            } 
            
            if sum < target {
                // If the sum is less than the target, continue exploring combinations
                // [.., last, last] and [.., last + 1] if last + 1 < candidates.len()
                let last = indices.pop();
                if let Some(last) = last {
                    if last + 1 < candidates.len() {
                        let mut indices_clone = indices.clone();
                        indices_clone.push(last + 1);
                        stack.push((indices_clone, sum - candidates[last] + candidates[last + 1]));
                    }
                    indices.push(last);
                    indices.push(last);
                    stack.push((indices, sum + candidates[last]));
                }
            }
        }

        result
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let answer = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(Solution::combination_sum(candidates, target), answer);
    }

    #[test]
    fn example2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let answer: Vec<Vec<i32>> = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(Solution::combination_sum(candidates, target), answer);
    }

    #[test]
    fn example3() {
        let candidates = vec![2];
        let target = 1;
        let answer = Vec::<Vec<i32>>::new();
        assert_eq!(Solution::combination_sum(candidates, target), answer);
    }
}
