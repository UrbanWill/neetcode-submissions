impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut subset = Vec::new();
        Self::dfs(&nums, 0, 0, target, &mut subset, &mut res);
        res
    }
    fn dfs(nums: &Vec<i32>, i: usize, sum: i32, target: i32, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if sum > target || i >= nums.len() {
            return;
        }

        if sum == target {
            res.push(subset.clone());
            return
        }

        subset.push(nums[i]);
        Self::dfs(nums, i, sum + nums[i], target, subset, res);

        subset.pop();
        Self::dfs(nums, i + 1, sum, target, subset, res);
    }
}
