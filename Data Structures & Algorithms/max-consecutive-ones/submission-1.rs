impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;

        nums.iter()
            .fold(0, |acc, num| {
                if *num == 1 {count +=1} else {count = 0}
                acc.max(count)
            })

    }
}
