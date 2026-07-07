use std::cmp::max;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut max_so_far = -1;

        for i in (0..arr.len()).rev() {
            let temp = arr[i];
            arr[i] = max_so_far;
            max_so_far = max(temp, max_so_far);
        }
        arr
    }
}
