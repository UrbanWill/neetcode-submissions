impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut subset = Vec::new();
        Self::dfs(&nums,0, &mut subset, &mut res);
        res
    }

    fn dfs(nums: &Vec<i32>, i: usize, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if i >= nums.len() {
            res.push(subset.clone());
        } else {
            subset.push(nums[i]);
            Self::dfs(nums, i + 1, subset, res);
            subset.pop();
            Self::dfs(nums, i + 1, subset, res);
        }
    }
}
