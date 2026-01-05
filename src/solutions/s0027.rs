/**
* 27. Remove Element
*
* Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed.

*
* Example 1:
* Input: nums = [3,2,2,3], val = 3
* Output: 2, nums = [2,2,_,_]
*
* Example 2:
* Input: nums = [0,1,2,2,3,0,4,2], val = 2
* Output: 5, nums = [0,1,4,0,3,_,_,_]

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cursor = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[cursor] = nums[i];
                cursor += 1;
            }
        }
        cursor as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [3,2,2,3], val = 3
        // Expected: 2, nums = [2,2,_,_]
        let mut nums = vec![3, 2, 2, 3];
        let k = Solution::remove_element(&mut nums, 3);
        assert_eq!(k, 2);
        assert_eq!(nums[..k as usize].to_vec(), vec![2, 2]);
    }

    #[test]
    fn example_2() {
        // Input: nums = [0,1,2,2,3,0,4,2], val = 2
        // Expected: 5, nums = [0,1,4,0,3,_,_,_]
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k = Solution::remove_element(&mut nums, 2);
        assert_eq!(k, 5);
        let mut out = nums[..k as usize].to_vec();
        out.sort_unstable();
        let mut expected = vec![0, 1, 4, 0, 3];
        expected.sort_unstable();
        assert_eq!(out, expected);
    }
}
