impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        // ans.extend_from_slice(&nums); // this works, but not as controlled and explicit.
        let mut ans: Vec<i32> = Vec::with_capacity(nums.len() * 2);
        
        ans.extend_from_slice(&nums);
        ans.extend_from_slice(&nums);

        ans
    }
}
