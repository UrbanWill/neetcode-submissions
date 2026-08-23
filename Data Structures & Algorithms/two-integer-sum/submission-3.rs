use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = Vec::new();

        for i in 0..nums.len() {
            let compliment = target - nums[i];

            if let Some(cmp_index) = map.get(&compliment) {
                res.push(*cmp_index as i32);
                res.push(i as i32);
            } else {
                map.insert(nums[i], i);
            }
        }


        res
    }
}
