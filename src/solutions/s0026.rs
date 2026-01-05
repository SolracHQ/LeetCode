/**
* 26. Remove Duplicates from Sorted Array
*
* Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once.

*
* Example 1:
* Input: nums = [1,1,2]
* Output: 2, nums = [1,2,_]
*
* Example 2:
* Input: nums = [0,0,1,1,1,2,2,3,3,4]
* Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as _
    }

    // I dont remember why I wrote this assembly version, and I dont comment it either.
    // But it works correctly, I really don't know how 😅
    pub fn _remove_duplicates_asm(nums: &mut Vec<i32>) -> i32 {
        let mut current_index: usize = 0;
        unsafe {
            std::arch::asm!(
                "mov r11, 0",
                "mov r8, 1",
                "4:",
                "cmp r8, r9",
                "jge 3f",
                "xor rax, rax",
                "mov eax, [r10 + 4*r8]",
                "cmp eax, [r10 + 4*r11]",
                "je 2f",
                "inc r11",
                "mov [r10 + 4*r11], eax",
                "2:",
                "inc r8",
                "jmp 4b",
                "3:",
                "mov [r12], r11",
                inout("r9") nums.len() => _,
                inout("r10") nums.as_mut_ptr() => _,
                inout("r12") &mut current_index => _,
            );
        }
        (current_index + 1) as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: nums = [1,1,2]
        // Expected: 2, nums = [1,2,_]
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
    }

    #[test]
    fn example_2() {
        // Input: nums = [0,0,1,1,1,2,2,3,3,4]
        // Expected: 5, nums = [0,1,2,3,4,_,_,_,_,_]
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
    }
}
