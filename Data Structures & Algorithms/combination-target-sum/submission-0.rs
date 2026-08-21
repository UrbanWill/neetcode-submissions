impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut subset: Vec<i32> = Vec::new();
        Self::dfs(&nums, 0, &mut subset, &mut res, target, 0);
        res
    }

    fn dfs(nums: &Vec<i32>, i: usize, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, target: i32, total: i32) {
        if i >= nums.len() || total > target {
            return;
        } 

        if total == target {
            res.push(subset.clone());
            return
        }

        subset.push(nums[i]);
        Self::dfs(nums, i, subset, res, target, total + nums[i]);
        subset.pop();
        Self::dfs(nums, i + 1, subset, res, target, total);
        
    }
}
