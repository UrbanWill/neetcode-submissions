impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut w = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[w] = nums[i];
                w += 1;
            }
        }

        w as i32
    }
}
