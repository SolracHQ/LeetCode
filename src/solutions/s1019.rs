/**
 * 1019. Squares of a Sorted Array
 *
 * Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
 *
 * Example 1:
 * Input: nums = [-4,-1,0,3,10]
 * Output: [0,1,9,16,100]
 *
 * Example 2:
 * Input: nums = [-7,-3,2,3,11]
 * Output: [4,9,9,49,121]
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // Strategy: Two pointers
        // 1. Find the first non-negative number's index
        //   Time O(n) using linear search, I can improve it with binary search but I'm to laizy and anyway the overall time complexity is O(n)
        // 2. Use two pointers to merge the squares of negative and non-negative parts
        //   Time complexity: O(n)
        // Overall time complexity: O(2n) = O(n)

        // Find the first non-negative number
        let first_posive = match nums.iter().position(|&x| x >= 0) {
            Some(pos) => pos,
            None => nums.len(),
        };

        // Preallocate result vector (only 1 allocation)
        let mut result = Vec::with_capacity(nums.len());

        // I'm treating 0 as positive so the negative pointer starts from first_posive - 1
        let mut neg_index = first_posive as isize - 1;
        // The positive pointer starts from first_posive
        let mut pos_index = first_posive as isize;

        loop {
            // This case covers 0 length input as well
            if neg_index < 0 && pos_index >= nums.len() as isize {
                break;
            }
            // If negative pointer is out of bounds, take from positive side
            if neg_index < 0 {
                result.push(nums[pos_index as usize] * nums[pos_index as usize]);
                pos_index += 1;
                continue;
            }
            // If positive pointer is out of bounds, take from negative side
            if pos_index >= nums.len() as isize {
                result.push(nums[neg_index as usize] * nums[neg_index as usize]);
                neg_index -= 1;
                continue;
            }
            // Both pointers are in bounds, take the smaller square
            if nums[neg_index as usize].abs() < nums[pos_index as usize] {
                result.push(nums[neg_index as usize] * nums[neg_index as usize]);
                neg_index -= 1;
            } else {
                result.push(nums[pos_index as usize] * nums[pos_index as usize]);
                pos_index += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [-4,-1,0,3,10]
        // Expected: [0,1,9,16,100]
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn example_2() {
        // Input: nums = [-7,-3,2,3,11]
        // Expected: [4,9,9,49,121]
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
