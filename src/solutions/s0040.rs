/// 40. Combination Sum II
/// https://leetcode.com/problems/combination-sum-ii/
///
/// Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
/// Each number in candidates may only be used once in the combination.
/// Note:
/// All numbers (including target) will be positive integers.
/// The solution set must not contain duplicate combinations.
struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();

        let mut result: Vec<Vec<i32>> = vec![];
        let mut stack = vec![(vec![0], candidates[0])];

        while let Some((mut indices, sum)) = stack.pop() {
            if sum == target {
                result.push(indices.iter().map(|indice| candidates[*indice]).collect());
                continue;
            }

            if sum < target {
                if let Some(last) = indices.pop() {

                    let mut next = last + 1;

                    if next >= candidates.len() {
                        continue;
                    }

                    let mut new_indices = indices.clone();
                    new_indices.push(last);
                    new_indices.push(next);
                    stack.push((new_indices, sum + candidates[next]));

                    while next < candidates.len() && candidates[next] == candidates[next - 1] {
                        next += 1;
                    }

                    if next >= candidates.len() {
                        continue;
                    }

                    indices.push(next);
                    stack.push((indices, sum + candidates[next] - candidates[last]));
                }
            }
        }

        result
    }
}

mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
                .into_iter()
                .rev()
                .collect::<Vec<Vec<i32>>>()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
                .into_iter()
                .rev()
                .collect::<Vec<Vec<i32>>>()
        );
    }
}
