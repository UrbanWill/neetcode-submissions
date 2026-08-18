impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let m = (l + r) / 2;
            let cur = nums[m as usize];

            if cur < target {
                l = m + 1;
            } else if cur > target {
                r = m - 1;
            } else {
                return m;
            }
        }
        -1
    }
}
