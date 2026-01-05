/**
* 40. Combination Sum II
*
* Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
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

#[cfg(test)]
mod test {
    use super::Solution;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for inner in v.iter_mut() {
            inner.sort_unstable();
        }
        v.sort_unstable();
        v
    }

    #[test]
    fn test_case_1() {
        // Example: candidates = [10,1,2,7,6,1,5], target = 8
        let result = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        let expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        assert_eq!(normalize(result), normalize(expected));
    }
}
