/**
* 39. Combination Sum
*
* Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target.

*
* Example 1:
* Input: candidates = [2,3,6,7], target = 7
* Output: [[2,2,3],[7]]
*
* Example 2:
* Input: candidates = [2,3,5], target = 8
* Output: [[2,2,2,2],[2,3,3],[3,5]]
*
* Example 3:
* Input: candidates = [2], target = 1
* Output: []

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
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
    fn example_1() {
        // Input: candidates = [2,3,6,7], target = 7
        // Expected: [[2,2,3],[7]]
        let result = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn example_2() {
        // Input: candidates = [2,3,5], target = 8
        // Expected: [[2,2,2,2],[2,3,3],[3,5]]
        let result = Solution::combination_sum(vec![2, 3, 5], 8);
        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn example_3() {
        // Input: candidates = [2], target = 1
        // Expected: []
        let result: Vec<Vec<i32>> = Solution::combination_sum(vec![2], 1);
        assert!(normalize(result).is_empty());
    }
}
