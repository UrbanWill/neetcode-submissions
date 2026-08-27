impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut sub = Vec::new();
        Self::dfs(&nums, &mut sub, &mut res, 0);

        res
    }

    fn dfs(nums: &Vec<i32>, sub: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, i: usize) {
        if i >= nums.len() {
            res.push(sub.clone());
        } else {
            sub.push(nums[i]);
            Self::dfs(nums, sub, res, i + 1);

            sub.pop();
            Self::dfs(nums, sub, res, i + 1);
        }
    }
}
