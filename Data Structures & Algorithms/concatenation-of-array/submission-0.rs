impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        [nums.as_slice(), nums.as_slice()].concat()
    }
}
