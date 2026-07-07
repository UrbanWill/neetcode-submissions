impl Solution {
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;

    nums.retain(|&num| {
        if num == val {
            false
        } else {
            k += 1;
            true
        }
    });

    k
}



}
