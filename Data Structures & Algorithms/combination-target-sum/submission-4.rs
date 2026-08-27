impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sub = Vec::new();
        let mut res = Vec::new();
        Self::dfs(&nums, &mut sub, &mut res, target, 0, 0);

        res
    }
    
    fn dfs(nums: &Vec<i32>, sub: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, target: i32, sum: i32, i: usize) {
        if i >= nums.len() || sum > target {
            return;
        }

        if sum == target {
            res.push(sub.clone());
            return;
        }

        sub.push(nums[i]);
        Self::dfs(nums, sub, res, target, sum + nums[i], i);

        sub.pop();
        Self::dfs(nums, sub, res, target, sum, i + 1);
    }
}
