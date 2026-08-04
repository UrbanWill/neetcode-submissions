impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;

        for i in (0..arr.len()).rev() {
            let temp = arr[i];
            arr[i] = max;
            max = max.max(temp);
        }

        arr
    }
}
