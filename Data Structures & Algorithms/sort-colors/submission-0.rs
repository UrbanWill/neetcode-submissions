impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count = [0; 3];

        for &num in nums.iter() {
            count[num as usize] += 1;
        }

        let mut index = 0;

        for i in 0..3 {
            while count[i] > 0 {
                nums[index] = i as i32;
                count[i] -= 1;
                index += 1; 
            }
        }
    }
}
